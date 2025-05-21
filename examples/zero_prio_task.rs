#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

use core::marker::PhantomData;

pub struct NotSend {
    _0: PhantomData<*const ()>,
}

#[rtic::app(device = stm32, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        x: NotSend,
    }

    #[local]
    struct Local {
        y: NotSend,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        async_task::spawn().unwrap();
        async_task2::spawn().unwrap();

        (
            Shared {
                x: NotSend { _0: PhantomData },
            },
            Local {
                y: NotSend { _0: PhantomData },
            },
        )
    }

    #[task(priority = 0, shared = [x], local = [y])]
    async fn async_task(_: async_task::Context) {
        info!("Async task running");
    }
    #[task(priority = 0, shared = [x])]
    async fn async_task2(_: async_task2::Context) {
        info!("Async task 2 running");
    }
}
