#![no_std]
#![no_main]

use esp_hal::{clock, gpio, main, time};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_println::println!("panicked");
    esp_hal::system::software_reset();
}

#[main]
fn main() -> ! {
    esp_println::println!("starting main");
    let config = esp_hal::Config::default().with_cpu_clock(clock::CpuClock::default());
    let peripherals = esp_hal::init(config);

    let mut led = gpio::Output::new(
        peripherals.GPIO5,
        gpio::Level::Low,
        gpio::OutputConfig::default(),
    );
    esp_println::println!("{:?}", led);

    loop {
        esp_println::println!("toggling led");
        led.toggle();

        let delay_start = time::Instant::now();
        while delay_start.elapsed() < time::Duration::from_millis(500) {}
    }
}
