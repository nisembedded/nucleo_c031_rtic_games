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

        foo::spawn(1, 2).unwrap();
        assert!(foo::spawn(3, 4).is_err()); // The capacity of the task is 1

        (Shared {}, Local {})
    }

    #[task]
    async fn foo(_: foo::Context, x: i32, y: i32) {
        info!("Foo task running with x: {}, y: {}", x, y);
    }
}
