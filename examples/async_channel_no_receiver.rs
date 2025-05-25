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
    type Sender<T> = channel::Sender<'static, T, CAPACITY>;
    type Receiver<T> = channel::Receiver<'static, T, CAPACITY>;


    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        let (tx, _rx) = make_channel!(u32, CAPACITY);

        sender1::spawn(tx).ok();

        (Shared {}, Local {})
    }

    #[task]
    async fn sender1(_: sender1::Context, mut tx: Sender<u32>) {
        info!("Sender 1 started");

        // This will block forever since there is no receiver
        info!("Sender 1 sending: 1 {:?}", tx.send(1).await);

        info!("Sender 1 finished");
    }
}