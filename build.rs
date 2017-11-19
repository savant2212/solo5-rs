use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // make solo5 virtio
    Command::new("make").args(&[ "-C", "solo5", "ukvm" ]).status().unwrap();

    // create static lib
    Command::new("ar").arg(&"crus").arg(&format!("{}/libsolo5.a", out_dir)).arg(&"solo5/kernel/ukvm/solo5.o").status().unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=solo5");
}
