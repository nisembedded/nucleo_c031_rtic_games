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
        shared: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        foo::spawn().unwrap();

        (Shared { shared: 0 }, Local {})
    }

    #[task(shared = [shared])]
    async fn foo(mut c: foo::Context) {
        info!("A");
        c.shared.shared.lock(|shared| {
            *shared += 1;

            bar::spawn().unwrap();

            info!("B - shared: {}", *shared);

            baz::spawn().unwrap();
        });
        info!("E");
    }

    #[task(shared = [shared], priority = 2)]
    async fn bar(mut c: bar::Context) {
        let shared = c.shared.shared.lock(|shared| {
            *shared += 1;

            *shared
        });

        info!("D - shared: {}", shared);
    }

    #[task(priority = 3)]
    async fn baz(_: baz::Context) {
        info!("C");
    }
}
