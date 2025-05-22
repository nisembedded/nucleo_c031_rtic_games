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
    struct Shared {}

    #[local]
    struct Local {
        local_to_foo: i64,
        local_to_bar: i64,
        local_to_idle: i64,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        info!("System initialized");
        foo::spawn().unwrap();
        bar::spawn().unwrap();

        (Shared {}, Local {
            local_to_foo: 0,
            local_to_bar: 0,
            local_to_idle: 0,
        })
    }

    #[idle(local = [local_to_idle])]
    fn idle(cx: idle::Context) -> ! {
        let local_to_idle = cx.local.local_to_idle;
        *local_to_idle += 1;
        info!("Idle task running, local_to_idle: {}", *local_to_idle);
        loop {
            cortex_m::asm::wfi();// Do something with local_to_idle
        }
    }

    #[task(local = [local_to_foo], priority = 1)]
    async fn foo(cx: foo::Context) {
        let local_to_foo = cx.local.local_to_foo;
        *local_to_foo += 1;
        info!("Foo task running, local_to_foo: {}", *local_to_foo);
    }

    #[task(local = [local_to_bar], priority = 1)]
    async fn bar(cx: bar::Context) {
        let local_to_bar = cx.local.local_to_bar;
        *local_to_bar += 1;
        info!("Bar task running, local_to_bar: {}", *local_to_bar);
    }
}
