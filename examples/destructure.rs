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
    struct Shared {
        a: u32,
        b: u32,
        c: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        foo::spawn().unwrap();
        bar::spawn().unwrap();

        (Shared { a: 0, b: 1, c: 2 }, Local {})
    }

    // Direct destructure
    #[task(shared = [&a, &b, &c], priority = 1)]
    async fn foo(cx: foo::Context) {
        let a = cx.shared.a;
        let b = cx.shared.b;
        let c = cx.shared.c;
        info!("foo: a = {}, b = {}, c = {}", a, b, c);
    }

    // De-structure-ing syntax
    #[task(shared = [&a, &b, &c], priority = 1)]
    async fn bar(cx: bar::Context) {
        let bar::SharedResources { a, b, c, .. } = cx.shared;
        info!("bar: a = {}, b = {}, c = {}", a, b, c);
    }
}
