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

    const CAPACITY: usize = 5;
    type Sender<T> = channel::Sender<'static, T, CAPACITY>;
    type Receiver<T> = channel::Receiver<'static, T, CAPACITY>;


    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        let (tx, rx) = make_channel!(u32, CAPACITY);
        receiver::spawn(rx).ok();
        sender1::spawn(tx.clone()).ok();
        sender2::spawn(tx.clone()).ok();
        sender3::spawn(tx).ok();

        (Shared {}, Local {})
    }

    #[task]
    async fn receiver(_: receiver::Context, mut rx: Receiver<u32>) {
        info!("Receiver started");

        while let Ok(value) = rx.recv().await {
            info!("Received value: {}", value);
        }

        info!("Receiver finished");
    }

    #[task]
    async fn sender1(_: sender1::Context, mut tx: Sender<u32>) {
        info!("Sender 1 started");

        tx.send(1).await.ok();

        info!("Sender 1 finished");
    }

    #[task]
    async fn sender2(_: sender2::Context, mut tx: Sender<u32>) {
        info!("Sender 2 started");

        tx.send(2).await.ok();
        
        info!("Sender 2 finished");
    }
    
    #[task]
    async fn sender3(_: sender3::Context, mut tx: Sender<u32>) {
        info!("Sender 3 started");

        tx.send(3).await.ok();

        info!("Sender 3 finished");
    }
}