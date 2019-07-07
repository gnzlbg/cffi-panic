# Single-threaded callback cancellation using setjmp/longjmp

Here we write a Rust callback, that is passed to and gets called by the C API.
This callback is able to report errors to the Rust code by doing a longjmp in C
over the C API code.

We can only do this if the API is single-threaded, that is, if the callback is
guaranteed to get executed in the same thread in which we call the C API.

Also, the C API needs to guarantee that a `longjmp` out of it is ok, since it is
trivial for a `longjmp` to create any sort of issues, e.g., due to resource
leaks:


```c
void the_worst(callback_t callback) {
    void* some_resource = /* malloc(1000), pthread_mutex_lock, fopen, ...*/;
    callback();
    // do some stuff with some_resoruce
    free(some_resource);
}
```

When in doubt, prefer to just `abort` instead of trying to cancel the operation
introducing undefined behavior in the process. 

If you need to return an error context to the caller, you can use a
`thread_local` to communicate it between the callback and the caller. 
