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

    #[init(local = [x: u32 = 0])]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Pends the USART1 interrupt but its handler won't run until
        // `init` return because the interrupt is not enabled yet

        // Cortex-M peripherals
        let _core = cx.core;
        // STM32 peripherals
        let _peripherals = cx.device;
        // Local variables
        let _x = cx.local.x;
        // Access to the critical section token
        // to indicate that we are in a critical section
        let _cs_token = cx.cs;

        info!("System initialized");

        (Shared {}, Local {})
    }
}