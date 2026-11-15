use std::env;
use std::process::{Command, exit};

fn main() {
    let bios_path = env!("BIOS_PATH");

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-drive")
        .arg(format!("format=raw,file={bios_path}"));
    cmd.arg("-serial").arg("mon:stdio");
    cmd.arg("-device")
        .arg("isa-debug-exit,iobase=0xf4,iosize=0x04");

    if env::var_os("DISPLAY").is_none() && env::var_os("WAYLAND_DISPLAY").is_none() {
        cmd.arg("-display").arg("none");
    }

    cmd.args(env::args().skip(1));

    let status = cmd.status().expect("failed to start qemu-system-x86_64");

    match status.code().unwrap_or(1) {
        0x10 | 0x21 => {}
        0x11 | 0x23 => exit(1),
        _ => exit(2),
    }
}
