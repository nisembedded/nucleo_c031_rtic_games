#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

#[rtic::app(device = stm32, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        #[lock_free]
        counter: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        rtic::pend(stm32::Interrupt::USART1);

        (Shared { counter: 0 }, Local {})
    }

    #[task(binds = USART1, shared = [counter])]
    fn foo(c: foo::Context) {
        rtic::pend(stm32::Interrupt::USART2);

        *c.shared.counter += 1;
        let counter = c.shared.counter;
        info!("USART1 task called, counter: {}", counter);
    }

    #[task(binds = USART2, shared = [counter])]
    fn bar(c: bar::Context) {
        rtic::pend(stm32::Interrupt::USART1);
        *c.shared.counter += 1;
        let counter = c.shared.counter;
        info!("USART2 task called, counter: {}", counter);
    }
}