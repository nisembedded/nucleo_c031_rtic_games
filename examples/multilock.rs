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
        shared1: u32,
        shared2: u32,
        shared3: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        locks::spawn().unwrap();

        (
            Shared {
                shared1: 0,
                shared2: 0,
                shared3: 0,
            },
            Local {},
        )
    }

    #[task(shared = [shared1, shared2, shared3])]
    async fn locks(mut c: locks::Context) {
        let (s1, s2, s3) = (c.shared.shared1, c.shared.shared2, c.shared.shared3);
        (s1, s2, s3).lock(|s1, s2, s3| {
            *s1 += 1;
            *s2 += 1;
            *s3 += 1;

            info!("Multiple locks - shared1: {}, shared2: {}, shared3: {}", *s1, *s2, *s3);
        });
    }
}
