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
    struct Local {
        sender: Sender<u32>,
    }

    const CAPACITY: usize = 2;
    type Sender<T> = channel::Sender<'static, T, CAPACITY>;
    type Receiver<T> = channel::Receiver<'static, T, CAPACITY>;


    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        let (tx, rx) = make_channel!(u32, CAPACITY);

        receiver::spawn(rx).ok();
        sender1::spawn(tx.clone()).ok();

        (Shared {}, Local { sender: tx })
    }

    #[task(priority = 2)]
    async fn receiver(_: receiver::Context, mut rx: Receiver<u32>) {
        info!("Receiver started");

        while let Ok(value) = rx.recv().await {
            info!("Received value: {}", value);
        }
    }

    #[task]
    async fn sender1(_: sender1::Context, mut tx: Sender<u32>) {
        info!("Sender 1 started");

        // This will block forever since there is no receiver
        info!("Sender 1 sending: 1 {:?}", tx.send(1).await);

        info!("Sender 1 finished");
        rtic::pend(stm32::Interrupt::USART2);
    }

    #[task(binds = USART2, local = [sender], priority = 1)]
    fn usart2_handler(cx: usart2_handler::Context) {
        info!("USART2 interrupt handler called");

        // Attempt to send a value through the channel
        cx.local.sender.try_send(42).ok();
    }
}