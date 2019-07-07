fn main() {
    let mut build = cc::Build::new();
    build.file("the_worst_support.cpp");
    build.flag("-fexceptions");
    build.cpp(true);
    build.compile("the_worst_support");

    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=the_worst.cp");
    println!("cargo:rerun-if-changed=the_worst_support.cp");
    println!("cargo:root={}", std::env::var("OUT_DIR").unwrap());
}
