#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

#[rtic::app(device = stm32, peripherals = true, dispatchers = [USART1, USART2])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        key: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        foo::spawn().unwrap();
        bar::spawn().unwrap();

        (Shared { key: 0xdeadbeef }, Local {})
    }
    #[task(shared = [&key])]
    async fn foo(cx: foo::Context) {
        let key: &u32 = cx.shared.key;
        info!("foo(key = {:#x})", key);
    }

    #[task(priority = 2, shared = [&key])]
    async fn bar(cx: bar::Context) {
        info!("bar(key = {:#x})", cx.shared.key);
    }

}