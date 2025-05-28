#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

#[rtic::app(device = stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use super::*;
    use rtic_sync::{channel, make_channel};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    const CAPACITY: usize = 1;
    type Receiver<T> = channel::Receiver<'static, T, CAPACITY>;


    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        let (_tx, rx) = make_channel!(u32, CAPACITY);

        receiver::spawn(rx).ok();

        (Shared {}, Local {})
    }

    #[task]
    async fn receiver(_: receiver::Context, mut rx: Receiver<u32>) {
        info!("Receiver started");

        info!("Received value: {:?}", rx.recv().await);
    }
}