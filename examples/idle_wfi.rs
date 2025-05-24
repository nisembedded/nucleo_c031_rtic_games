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
    fn init(mut cx: init::Context) -> (Shared, Local) {
        info!("System initialized");

        cx.core.SCB.set_sleepdeep(); // Enable deep sleep mode

        (Shared {}, Local {})
    }

    #[idle(local = [x: u32 = 0])]
    fn idle(cx: idle::Context) -> ! {
        // This is the idle task, it runs when no other task is ready to run.
        // It can be used to put the CPU into a low-power state or perform background tasks.

        // Local variables
        let _x = cx.local.x;

        info!("Idle task running");

        loop {
            rtic::export::wfi(); // Wait for interrupt (WFI)
        }
    }
}