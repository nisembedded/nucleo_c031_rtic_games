#![no_main]
#![no_std]

use stm32c0xx_hal as hal;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use hal::stm32;

#[rtic::app(device = stm32, peripherals = true, dispatchers = [USART1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
    }

    #[init(local = [a: u32 = 0])]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");

        foo::spawn().unwrap();

        (Shared {}, Local {
        })
    }

    #[idle(local = [a: u32 = 0])]
    fn idle(cx: idle::Context) -> ! {
        let local_to_idle = cx.local.a;
        *local_to_idle += 1;
        info!("Idle task running, local_to_idle: {}", *local_to_idle);
        loop {
            cortex_m::asm::wfi();// Do something with local_to_idle
        }
    }

    #[task(local = [a: u32 = 0], priority = 1)]
    async fn foo(cx: foo::Context) {
        let local_to_foo = cx.local.a;
        *local_to_foo += 1;
        info!("Foo task running, local_to_foo: {}", *local_to_foo);
    }
}
