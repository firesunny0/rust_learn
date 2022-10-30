#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::{asm, Peripherals};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f4::stm32f407 as device;

// #[panic_handler]
// fn my_panic_handler(_panic: &PanicInfo) -> ! {
//     loop {}
// }

fn my_delay() {
    //简陋的延迟函数
    let mut count = 2;
    while count > 0 {
        count = count - 1;
        let mut count2 = 120000;
        while count2 > 0 {
            count2 = count2 - 1;
        }
    }
}

#[entry]
fn main() -> ! {
    let mut p = device::Peripherals::take().unwrap();
    let rcc = p.RCC;
    rcc.ahb1enr.modify(|_, w| w.gpiofen().set_bit());
    rcc.ahb1enr.write(|w| w.gpiofen().enabled());
    p.GPIOF.moder.modify(|_, w| unsafe { w.moder9().bits(1) });
    hprintln!("stm32f4 enter loop!").unwrap();

    loop {
        p.GPIOF.odr.modify(|_, w| w.odr9().low()); //灯亮
        hprintln!("Light is on!").unwrap();
        my_delay();
        p.GPIOF.odr.modify(|_, w| w.odr9().high()); //灯灭
        hprintln!("Light is off!").unwrap();
        my_delay();
    }
}
