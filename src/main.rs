#![no_main]
#![no_std]

use cortex_m_rt::{exception, ExceptionFrame};
use rtt_target::{rprintln, rtt_init, set_print_channel};
use stm32f4xx_hal::{prelude::*, stm32::TIM4};
use stm32f4xx_hal::timer::Timer;

mod helpers;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        global_timer: Timer<TIM4>,
    }

    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
        // Cortex-M peripherals
        let _cp: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let dp: stm32f4xx_hal::stm32::Peripherals = cx.device;

        let channels = rtt_init! {
            up: {
                0: {
                    size: 1024
                    mode: NoBlockSkip
                    name: "Terminal"
                }
            }
        };

        set_print_channel(channels.up.0);

        rprintln!("Initializing!");

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(168.mhz()).freeze();

        let mut global_timer = Timer::tim4(dp.TIM4, 1u32.hz(), clocks);
        global_timer.listen(stm32f4xx_hal::timer::Event::TimeOut);

        init::LateResources {
            global_timer
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        stm32f4xx_hal::stm32::NVIC::pend(stm32f4xx_hal::stm32::Interrupt::EXTI0);

        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = TIM4, resources = [global_timer])]
    fn on_global_timer(cx: on_global_timer::Context) {
        let global_timer: &mut Timer<TIM4> = cx.resources.global_timer;
        global_timer.clear_interrupt(stm32f4xx_hal::timer::Event::TimeOut);

        rprintln!("On global timer tick");
    }

    #[task(binds = EXTI0)]
    fn exti(cx: exti::Context) {
        rprintln!("exti");
    }
};


#[exception]
fn DefaultHandler(irq: i16) -> ! {
    debug_only!({
        panic!("Interrupt {} fired before it was set up first!", irq);
    });

    release_only!(panic!("Default handler"));

    loop {}
}

#[exception]
fn HardFault(frame: &ExceptionFrame) -> ! {
    debug_only!({
        panic!("Hardfault: {:?}", frame);
    });

    release_only!(panic!("Hardfault"));

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    rprintln!("{}", info);
    debugger_only!(cortex_m::asm::bkpt());

    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst)
    }
}
