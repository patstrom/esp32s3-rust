#![no_std]
#![no_main]

use core::ptr::write_volatile;
use esp_hal::main;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const LOW_POWER_MANAGEMENT_BASE: u32 = 0x6000_8000;
const SWD_WKEY_VALUE: u32 = 0x8F1D312A;
const RTC_CNTL_WDT_WKEY: u32 = 0x50d83aa1;

fn disable_super_watchdog() {
    const RTC_CNTL_RTC_SWD_WPROTECT_REG: *mut u32 =
        (LOW_POWER_MANAGEMENT_BASE + 0x00B8) as *mut u32;
    const RTC_CNTL_RTC_SWD_CONF_REG: *mut u32 = (LOW_POWER_MANAGEMENT_BASE + 0x00B4) as *mut u32;

    unsafe {
        write_volatile(RTC_CNTL_RTC_SWD_WPROTECT_REG, SWD_WKEY_VALUE);
        write_volatile(RTC_CNTL_RTC_SWD_CONF_REG, 1 << 30); // RTC_CNTL_SWD_DISABLE
        write_volatile(RTC_CNTL_RTC_SWD_WPROTECT_REG, 0);
    }
}

fn disable_rtc_watchdog() {
    const RTC_CNTL_RTC_WDTWPROTECT_REG: *mut u32 = (LOW_POWER_MANAGEMENT_BASE + 0x00B0) as *mut u32;
    const RTC_CNTL_RTC_WDTCONFIG0_REG: *mut u32 = (LOW_POWER_MANAGEMENT_BASE + 0x0098) as *mut u32;

    unsafe {
        write_volatile(RTC_CNTL_RTC_WDTWPROTECT_REG, RTC_CNTL_WDT_WKEY);
        write_volatile(RTC_CNTL_RTC_WDTCONFIG0_REG, 0);
        write_volatile(RTC_CNTL_RTC_WDTWPROTECT_REG, 0);
    }
}

const TIMG_BASE: u32 = 0x6001f000;
const TIMG_WDT_WKEY: u32 = 0x50D83AA1;

fn disable_task_watchdog() {
    const TIMG_WDTCONFIG0_REG: *mut u32 = (TIMG_BASE + 0x0048) as *mut u32;
    const TIMG_WDTWPROTECT_REG: *mut u32 = (TIMG_BASE + 0x0064) as *mut u32;

    unsafe {
        write_volatile(TIMG_WDTWPROTECT_REG, TIMG_WDT_WKEY);
        write_volatile(TIMG_WDTCONFIG0_REG, 0);
        write_volatile(TIMG_WDTWPROTECT_REG, 0);
    }
}

const GPIO_BASE: u32 = 0x6000_4000;

#[main]
fn main() -> ! {
    // The esp32s3 has a few watchdogs that reset the system if they are not fed. Instead of
    // feeding them we'll just disable them. It seems like it's actually only the rtc watchdog that
    // needs to be disabled for this to work though, dunno why.
    disable_super_watchdog();
    disable_rtc_watchdog();
    disable_task_watchdog();

    const GPIO_ENABLE_REG: *mut u32 = (GPIO_BASE + 0x0020) as *mut u32;
    const GPIO_OUT_REG: *mut u32 = (GPIO_BASE + 0x0004) as *mut u32;

    unsafe {
        write_volatile(GPIO_ENABLE_REG, 1 << 5); // Enable pin 5
    }

    let mut is_on = false;
    loop {
        unsafe {
            write_volatile(GPIO_OUT_REG, (is_on as u32) << 5);
        }

        spin_loop(800_000);

        is_on = !is_on;
    }
}

fn spin_loop(n: u32) {
    // This is a poor man's spin_loop. We use black_box to ensure it isn't optimized away.
    let inc = |i: u32| i + 1;

    let mut v = 0;
    for _ in 0..n {
        v = core::hint::black_box(inc(v));
    }
}
