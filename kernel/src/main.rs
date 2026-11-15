#![no_std]
#![no_main]

mod framebuffer;

use bootloader_api::{BootInfo, entry_point};
use core::fmt::Write;
use framebuffer::FrameBufferWriter;
use spin::Mutex;

/// Global writer so that the panic handler can also print to the screen.
pub static WRITER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info
        .framebuffer
        .take()
        .expect("bootloader did not provide a framebuffer");

    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

    let mut writer = FrameBufferWriter::new(buffer, info);
    writeln!(writer, "Hello World from Rust OS!").unwrap();

    *WRITER.lock() = Some(writer);

    halt();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // `try_lock` avoids deadlocking if the panic happened while `WRITER` was locked.
    if let Some(mut writer) = WRITER.try_lock() {
        if let Some(writer) = writer.as_mut() {
            let _ = writeln!(writer, "PANIC: {info}");
        }
    }

    halt();
}

fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
