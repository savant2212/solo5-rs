use std::env;
use std::process::Command;

#[cfg(feature = "ukvm")]
fn get_variant() -> &'static str {
    return "ukvm";
}

#[cfg(feature = "virtio")]
fn get_variant() -> &'static str {
    return "virtio";
}


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let variant = get_variant();

    // make solo5 virtio
    Command::new("make").args(&[ "-C", "solo5", &variant ]).status().unwrap();
    Command::new("cp").arg(&format!("solo5/kernel/{}/solo5.lds", variant)).arg(&format!("{}/solo5.lds",out_dir)).status().unwrap();
    // create static lib
    Command::new("ar").arg(&"crus").arg(&format!("{}/libsolo5.a", out_dir)).arg(&format!("solo5/kernel/{}/solo5.o", variant)).status().unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=solo5");
}
