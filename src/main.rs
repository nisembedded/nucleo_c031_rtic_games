#![no_std]
#![no_main]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::exti::Event;
use hal::gpio::*;
use hal::prelude::*;
use hal::stm32;
use hal::time::*;
use hal::timer::*;

#[rtic::app(device = stm32, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        timer: Timer<stm32::TIM17>,
    }

    #[local]
    struct Local {
        exti: stm32::EXTI,
        led: PA5<Output<PushPull>>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let mut rcc = ctx.device.RCC.constrain();
        let gpioa = ctx.device.GPIOA.split(&mut rcc);
        let gpioc = ctx.device.GPIOC.split(&mut rcc);

        let mut timer = ctx.device.TIM17.timer(&mut rcc);
        timer.start(Hertz::Hz(3).into_duration());
        timer.listen();

        let mut exti = ctx.device.EXTI;
        gpioc.pc13.listen(SignalEdge::Falling, &mut exti);

        info!("System initialized");
        info!("Press the button to start/stop the timer");
        info!("LED is on PA5");
        info!("Button is on PC13");
        info!("Timer is on TIM17");

        (
            Shared { timer },
            Local {
                exti,
                led: gpioa.pa5.into_push_pull_output(),
            },
        )
    }

    #[task(binds = TIM17, shared = [timer], local = [led])]
    fn timer_tick(mut ctx: timer_tick::Context) {
        ctx.local.led.toggle().ok();
        ctx.shared.timer.lock(|tim| tim.clear_irq());
    }

    #[task(binds = EXTI4_15, shared = [timer], local = [exti])]
    fn button_click(mut ctx: button_click::Context) {
        ctx.shared.timer.lock(|tim| {
            if tim.enabled() {
                tim.pause();
            } else {
                tim.resume();
            }
        });
        ctx.local.exti.unpend(Event::GPIO13);
        info!("Button clicked");
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
