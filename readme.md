# Handling errors in Rust->C->Rust FFI

This crate shows how to interface with C libraries that take callbacks from
Rust. In particular, we show how to write those callbacks in Rust, and how to
cancel the operation and/or propagate errors originating in those callbacks back
to Rust correctly.

The C API we are going to wrap is:

```c
typedef void (*callback_t)();
void the_worst(callback_t)
```

Notice that `callback_t` does not:

* return an error code that could be used to cancel the operation and gets
  propagated back to the caller.
* take a "callback context" where, e.g., an error could be stored that the
  caller could check.
  
C APIs accepting callbacks usually at least allow passing a callback context,
and often they allow both. Wrapping these is easy: just return an error code if
possible, and pass the error in the callback context otherwise.

Instead, here we are going to focus on `the_worst` possibly-designed C callback
APIs. They don't help us passing errors at all, and doing it right is going to
require hard work. 

There are two crates in this repository:

* `single_threaded_cancel_longjmp`: requires the C API to guarantee that the callback
  is executed in the same thread as the caller, and that it is safe for the
  callback to longjmp into the caller. Ideally, it would also guarantee that no
  resources are leaked in this case.
  
* `single_threaded_cancel_exception`: requires the C API to support C++
  exceptions, e.g., be compiled with `-fexceptions`, and support for the callback
  to throw an exception to cancel the operation.
