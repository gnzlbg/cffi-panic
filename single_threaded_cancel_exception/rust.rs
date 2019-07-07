extern "C" {
    fn the_worst_wrapper(callback: Option<extern "C" fn() -> ()>) -> i32;
    fn callback_error(error: i32);
}

thread_local! {
    pub static ERROR_CONTEXT: std::cell::Cell<&'static str>
        = std::cell::Cell::new("no error");
}

extern "C" fn my_callback() {
    println!("[my_callback]: enter");
    ERROR_CONTEXT.with(|f| {
        f.set("42 is not the answer");
    });
    unsafe { callback_error(42) }; 
}

fn main() { unsafe {
    println!("[main]: enter");
    match the_worst_wrapper(Some(my_callback)) {
        0 => println!("[main]: exit success"),
        v => ERROR_CONTEXT.with(|f| {
                println!("[main]: exit error: {} msg: {}", v, f.get());
        }),
    };
}}
