#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_semihosting::hprintln;

#[no_mangle]
pub static MY_DATA: u32 = 0xDEADBEEF;

const SYSTICK: u32 = 0xE000E010; // SysTick Base Address
const SYST_RVR: *mut u32 = (SYSTICK + 0x4) as *mut u32; // SysTick Reload Value Register
const SYST_CVR: *mut u32 = (SYSTICK + 0x8) as *mut u32; // SysTick Current Value Register
const SYST_CSR: *mut u32 = SYSTICK as *mut u32; // SysTick Control and Status Register

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    hprintln!("Hello, world!"); 

    unsafe {
        // Volatile read to ensure the data is considered used
        let _ = core::ptr::read_volatile(&MY_DATA as *const u32);
    } 

    systick_init();

    loop {
        // Poll the COUNTFLAG to check if SysTick has counted to 0 since the last RELOAD_VALUE
        if unsafe { SYST_CSR.read_volatile() } & (1 << 16) != 0 {
            // SysTick has counted down to 0, indicating a 1-second delay
            // Here you could toggle an LED or perform some other operation
            // For this example, we'll just continue looping
            hprintln!("Old school Blink!\n");
        }
    }
}

#[no_mangle]
fn systick_init() {
    // Disable SysTick timer and clear its current value
    unsafe {
        SYST_CSR.write_volatile(0);
        SYST_CVR.write_volatile(0);
    }

    // Set reload value for a 1 second delay assuming a 48MHz CPU clock
    // The SysTick is a 24-bit timer, so the reload value must not exceed 0x00FFFFFF.
    const RELOAD_VALUE: u32 = 16_000_000 - 1; // Adjust the clock speed accordingly
    unsafe {
        SYST_RVR.write_volatile(RELOAD_VALUE);
    }

    // Enable SysTick timer with processor clock
    unsafe {
        SYST_CSR.write_volatile(0x5);
    }
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: extern "C" fn() -> ! = _start;
