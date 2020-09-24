#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

//#[macro_use]
extern crate alloc;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use cortex_m_rt::{exception, ExceptionFrame};
use opl_driver::{
    hl::Note,
    hl::Opl2Error,
    hl::Rhythm,
    ll::{Bit, ShiftInterface},
};
use rtt_target::{rprintln, rtt_init, set_print_channel};
use sequencer::{Action, ActionPoint, Sequence};
use spi::{NoMiso, Spi};
use stm32f4xx_hal::{
    delay::Delay, gpio::gpioa::PA2, gpio::gpioa::PA3, gpio::gpioa::PA4, gpio::gpioa::PA5,
    gpio::gpioa::PA6, gpio::gpioa::PA7, gpio::Alternate, gpio::OpenDrain, gpio::Output,
    gpio::PushPull, gpio::AF5, hal::spi::MODE_0, spi, stm32::SPI1, timer::Timer,
};
use stm32f4xx_hal::{prelude::*, stm32::TIM4};

mod helpers;
mod sequencer;

type Led2Pin = PA6<Output<OpenDrain>>;

type Opl<S> = opl_driver::hl::Opl2<
    ShiftInterface<
        Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>,
        PA4<Output<PushPull>>,
        PA3<Output<PushPull>>,
        PA2<Output<PushPull>>,
        Delay,
    >,
    S,
>;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const FULL: u32 = 128;
const HALF: u32 = 64;
const QUARTER: u32 = 32;
const EIGHTH: u32 = 16;
const SIXTEENTH: u32 = 8;

const BPM: u32 = 120;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        global_timer: Timer<TIM4>,
        led_2: Led2Pin,
        opl: Opl<Rhythm>,
        music_sequence: Sequence<Opl<Rhythm>, Opl2Error>,
    }

    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
        // Cortex-M peripherals
        let cp: cortex_m::Peripherals = cx.core;

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

        // Initialize the heap
        const HEAP_SIZE: usize = 1024 * 100;
        unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

        rprintln!(
            "Heap setup at 0x{:x} with a size of {}KB",
            cortex_m_rt::heap_start() as usize,
            HEAP_SIZE as usize / 1024
        );

        let rcc = dp.RCC.constrain();
        const CLOCK_SPEED: u32 = 168_000_000;
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(CLOCK_SPEED.hz())
            .hclk(CLOCK_SPEED.hz())
            .pclk1((CLOCK_SPEED / 4).hz())
            .pclk2((CLOCK_SPEED / 2).hz())
            .freeze();

        rprintln!("Clock speed set at {}", CLOCK_SPEED);

        let gpioa = dp.GPIOA.split();

        let mut led_2: Led2Pin = gpioa.pa6.into_open_drain_output();
        led_2.set_high().unwrap();

        let ticks_per_second = BPM * QUARTER / 60;
        rprintln!("Music at {}({}) bpm and {} ticks per second", BPM, ticks_per_second * 60, ticks_per_second);
        let mut global_timer = Timer::tim4(dp.TIM4, ticks_per_second.hz(), clocks);
        global_timer.listen(stm32f4xx_hal::timer::Event::TimeOut);

        // Setup opl hardware
        let opl_spi = spi::Spi::spi1(
            dp.SPI1,
            (
                gpioa.pa5.into_alternate_af5(),
                spi::NoMiso,
                gpioa.pa7.into_alternate_af5(),
            ),
            MODE_0,
            1.mhz().into(),
            clocks,
        );
        let opl_a0 = gpioa.pa4.into_push_pull_output();
        let opl_latch = gpioa.pa3.into_push_pull_output();
        let opl_reset = gpioa.pa2.into_push_pull_output();
        let opl_delay = stm32f4xx_hal::delay::Delay::new(cp.SYST, clocks);

        let opl = opl_driver::hl::Opl2::new(opl_driver::ll::ShiftInterface::new(
            opl_spi, opl_a0, opl_latch, opl_reset, opl_delay,
        ));

        let mut opl = opl.initialize().unwrap().into_rhythm_mode().unwrap();

        opl.ll()
            .waveform_select_enable()
            .write(|w| w.waveform_select_enable(Bit::Set))
            .unwrap();

        #[rustfmt::skip]
        let mut music_sequence: Sequence<Opl<Rhythm>, Opl2Error> = Sequence::new(&[
            ActionPoint::new(0, Action::Custom { function: |opl| opl.setup_melody_instrument(0, opl_driver::instrument::presets::STRINGS1) }),

            ActionPoint::new(0, Action::PlayNote { channel: 0, value: Note::C(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::D(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::E(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::C(4), duration: QUARTER }),

            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::C(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::D(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::E(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::C(4), duration: QUARTER }),

            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::E(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::F(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::G(4), duration: HALF }),

            ActionPoint::new(HALF, Action::PlayNote { channel: 0, value: Note::E(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::F(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::G(4), duration: HALF }),

            ActionPoint::new(HALF, Action::PlayNote { channel: 0, value: Note::G(4), duration: EIGHTH }),
            ActionPoint::new(EIGHTH, Action::PlayNote { channel: 0, value: Note::A(4), duration: EIGHTH }),
            ActionPoint::new(EIGHTH, Action::PlayNote { channel: 0, value: Note::G(4), duration: EIGHTH }),
            ActionPoint::new(EIGHTH, Action::PlayNote { channel: 0, value: Note::F(4), duration: EIGHTH }),
            ActionPoint::new(EIGHTH, Action::PlayNote { channel: 0, value: Note::E(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::C(4), duration: QUARTER }),

            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::G(4), duration: EIGHTH }),
            ActionPoint::new(EIGHTH, Action::PlayNote { channel: 0, value: Note::A(4), duration: EIGHTH }),
            ActionPoint::new(EIGHTH, Action::PlayNote { channel: 0, value: Note::G(4), duration: EIGHTH }),
            ActionPoint::new(EIGHTH, Action::PlayNote { channel: 0, value: Note::F(4), duration: EIGHTH }),
            ActionPoint::new(EIGHTH, Action::PlayNote { channel: 0, value: Note::E(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::C(4), duration: QUARTER }),

            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::C(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::G(3), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::C(4), duration: HALF }),

            ActionPoint::new(HALF, Action::PlayNote { channel: 0, value: Note::C(4), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::G(3), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 0, value: Note::C(4), duration: HALF }),
        ]);

        #[rustfmt::skip]
        let bass_sequence: Sequence<Opl<Rhythm>, Opl2Error> = Sequence::new(&[
            ActionPoint::new(0, Action::Custom { function: |opl| opl.setup_melody_instrument(1, opl_driver::instrument::presets::GUITAR1) }),

            ActionPoint::new(FULL * 2, Action::PlayNote { channel: 1, value: Note::C(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::D(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::E(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::C(2), duration: QUARTER }),

            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::C(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::D(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::E(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::C(2), duration: QUARTER }),

            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::E(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::F(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::G(2), duration: HALF }),

            ActionPoint::new(HALF, Action::PlayNote { channel: 1, value: Note::E(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::F(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::G(2), duration: HALF }),

            ActionPoint::new(HALF, Action::PlayNote { channel: 1, value: Note::E(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::D(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::C(2), duration: HALF }),

            ActionPoint::new(HALF, Action::PlayNote { channel: 1, value: Note::E(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::D(2), duration: QUARTER }),
            ActionPoint::new(QUARTER, Action::PlayNote { channel: 1, value: Note::C(2), duration: QUARTER }),
        ]);

        music_sequence.merge(bass_sequence);

        init::LateResources {
            global_timer,
            led_2,
            opl,
            music_sequence,
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = TIM4, resources = [global_timer, led_2, opl, music_sequence])]
    fn on_global_timer(cx: on_global_timer::Context) {
        static mut COUNT: u32 = 0;

        let global_timer: &mut Timer<TIM4> = cx.resources.global_timer;
        let led_2: &mut Led2Pin = cx.resources.led_2;
        let opl: &mut Opl<Rhythm> = cx.resources.opl;
        let music_sequence: &mut Sequence<Opl<Rhythm>, Opl2Error> = cx.resources.music_sequence;

        global_timer.clear_interrupt(stm32f4xx_hal::timer::Event::TimeOut);
        led_2.toggle().unwrap();

        if !music_sequence.run(opl, *COUNT).unwrap() {
            cortex_m::asm::bkpt();
        }

        *COUNT = COUNT.wrapping_add(1);
    }
};

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("Alloc error: {:?}", layout);
}

#[exception]
fn DefaultHandler(irq: i16) -> ! {
    debug_only!({
        panic!("Interrupt {} fired while there's no handler for it.", irq);
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
