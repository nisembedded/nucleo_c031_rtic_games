#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

#[rtic::app(device = stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use super::*;
    use rtic_monotonics::systick::prelude::*;

    systick_monotonic!(Mono, 100);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        Mono::start(cx.core.SYST, 6_000_000);

        info!("System initialized");

        foo::spawn().ok();
        bar::spawn().ok();
        baz::spawn().ok();

        (Shared {}, Local {})
    }

    #[task]
    async fn foo(_: foo::Context) {
        info!("foo started");
        Mono::delay(100.millis()).await;
        info!("foo finished");
    }

    #[task]
    async fn bar(_: bar::Context) {
        info!("bar started");
        Mono::delay(200.millis()).await;
        info!("bar finished");
    }
    #[task]
    async fn baz(_: baz::Context) {
        info!("baz started");
        Mono::delay(300.millis()).await;
        info!("baz finished");
    }
}