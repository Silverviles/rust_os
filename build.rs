use std::path::PathBuf;

fn main() {
    // Set by cargo; build scripts must write their output files here.
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    // Set by cargo's artifact dependency feature:
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel").unwrap());

    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .expect("failed to create BIOS disk image");

    // Pass the disk image path to the builder binary.
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
