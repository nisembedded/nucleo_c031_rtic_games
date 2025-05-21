#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

#[rtic::app(device = stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        for _ in 0..3 {
            foo::spawn().unwrap();
            info!("Idle task running");
        }
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(priority = 1)]
    async fn foo(_: foo::Context) {
        info!("Foo task running");
    }
}
