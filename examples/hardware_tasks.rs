#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

#[rtic::app(device = stm32, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        // Pends the USART1 interrupt but its handler won't run until
        // `init` return because the interrupt is not enabled yet.
        rtic::pend(stm32::Interrupt::USART1);

        info!("System initialized");
        
        (
            Shared {},
            Local {
            },
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        info!("Idle task running");
        
        // Some backends provide a manual way of pending an interrupt.
        rtic::pend(stm32::Interrupt::USART1);
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = USART1, local = [times: u32 = 0])]
    fn usart1(cx: usart1::Context) {
        *cx.local.times += 1;
        info!("USART1 task called, times: {}", cx.local.times);
    }
}
