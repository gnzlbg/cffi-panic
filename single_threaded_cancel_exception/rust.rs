extern "C" {
    fn the_worst_wrapper(callback: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn my_callback_wrapper();
}

thread_local! {
    pub static ERROR_CONTEXT: std::cell::Cell<&'static str>
        = std::cell::Cell::new("no error");
}

#[no_mangle]
extern "C" fn my_callback() -> u32 {
    println!("[my_callback]: enter");
    ERROR_CONTEXT.with(|f| {
        f.set("42 is not the answer");
    });
    1
}

fn main() { unsafe {
    println!("[main]: enter");
    match the_worst_wrapper(Some(my_callback_wrapper)) {
        0 => println!("[main]: exit success"),
        v => ERROR_CONTEXT.with(|f| {
                println!("[main]: exit error: {} msg: {}", v, f.get());
        }),
    };
}}
