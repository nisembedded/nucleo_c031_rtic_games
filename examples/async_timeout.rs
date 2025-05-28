#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

use rtic_monotonics::systick::prelude::*;

systick_monotonic!(Mono, 100);

#[rtic::app(device = stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use super::*;
    use futures::{future::FutureExt, select_biased};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        Mono::start(cx.core.SYST, 6_000_000);

        info!("System initialized");

        foo::spawn().ok();

        (Shared {}, Local {})
    }

    #[task]
    async fn foo(_: foo::Context) {
        // ANCHOR: select biased
        // Call hal with short relative timeout using `select_biased`.
        select_biased! {
            v = hal_get(1).fuse() => info!("hal_get(1) returned: {}", v),
            _ = Mono::delay(200.millis()).fuse() => info!("Timeout after 200ms"),
        }

        // Cal hal with long relative timeout using `select_biased`.
        select_biased! {
            v = hal_get(2).fuse() => info!("hal_get(2) returned: {}", v),
            _ = Mono::delay(1000.millis()).fuse() => info!("Timeout after 1000ms"),
        }
        // ANCHOR_END: select biased

        // ANCHOR: timeout_after_basic
        // Call hal with short absolute timeout using `timeout_after`.
        match Mono::timeout_after(1000.millis(), hal_get(3)).await {
            Ok(v) => info!("hal_get(3) returned: {} at time {}", v, Mono::now()),
            Err(_) => info!("Timeout after 1000ms"),
        }
        // ANCHOR_END: timeout_after_basic

        // ANCHOR: timeout_at_basic
        // get the current time instance
        let mut instant = Mono::now();

        // do this 3 times
        for n in 0..3 {
            // absolute point in time without drift
            instant += 1000.millis();
            Mono::delay_until(instant).await;

            // absolute point in time for timeout
            let timeout = instant + 500.millis();
            info!("now is {:?}, timeout at {:?}", Mono::now(), timeout);

            match Mono::timeout_at(timeout, hal_get(n)).await {
                Ok(v) => info!("hal_get({}) returned: {}", n, v),
                Err(_) => info!("Timeout at {:?}", timeout),
            }
            // ANCHOR_END: timeout_at_basic
        }
    }
}

// Emulate some hal
async fn hal_get(n: u32) -> u32 {
    // emulate some delay time depended on `n`
    let d = 350.millis() + n * 100.millis();
    info!("the hal take a duration of {}ms", d.to_millis());
    Mono::delay(d).await;
    // emulate some result
    5 * n
}
