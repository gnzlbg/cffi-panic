fn main() {
    let mut build = cc::Build::new();
    build.file("the_worst_support.c");
    build.compile("the_worst_support");

    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=the_worst.c");
    println!("cargo:rerun-if-changed=the_worst_support.c");
    println!("cargo:root={}", std::env::var("OUT_DIR").unwrap());
}
