#![no_main]
#![no_std]

use cortex_m_rt::{exception, ExceptionFrame};
use rtt_target::{rprintln, rtt_init, set_print_channel};
use stm32f4xx_hal::{gpio::OpenDrain, gpio::Output, gpio::gpioa::PA6, gpio::gpioa::PA7, timer::Timer};
use stm32f4xx_hal::{prelude::*, stm32::TIM4};

mod helpers;

type Led2Pin = PA6<Output<OpenDrain>>;
type Led3Pin = PA7<Output<OpenDrain>>;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        global_timer: Timer<TIM4>,
        led_2: Led2Pin,
        led_3: Led3Pin,
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
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(168.mhz())
            .hclk(168.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())
            .freeze();

        let gpioa = dp.GPIOA.split();

        let mut led_2: Led2Pin = gpioa.pa6.into_open_drain_output();
        led_2.set_high().unwrap();
        let mut led_3: Led3Pin = gpioa.pa7.into_open_drain_output();
        led_3.set_high().unwrap();

        let mut global_timer = Timer::tim4(dp.TIM4, 1.hz(), clocks);
        global_timer.listen(stm32f4xx_hal::timer::Event::TimeOut);

        init::LateResources {
            global_timer,
            led_2,
            led_3
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = TIM4, resources = [global_timer, led_2])]
    fn on_global_timer(cx: on_global_timer::Context) {
        static mut COUNT: u32 = 0;

        let global_timer: &mut Timer<TIM4> = cx.resources.global_timer;
        global_timer.clear_interrupt(stm32f4xx_hal::timer::Event::TimeOut);

        let led_2: &mut Led2Pin = cx.resources.led_2;
        led_2.toggle().unwrap();

        rprintln!("On global timer tick: {}", COUNT);
        *COUNT = COUNT.wrapping_add(1);
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
