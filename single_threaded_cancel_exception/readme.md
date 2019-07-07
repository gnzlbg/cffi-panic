# Callback cancellation using exceptions

Here we write a Rust callback, that is passed to and gets called by the C API.
This callback is able to report errors to the Rust code by doing a longjmp in C
over the C API code.

We can only do this if the API is a C++ API, or a C API that can handle C++
exceptions, e.g., because it is compiled with `-fexceptions`.

This approach works even in a multi-threaded scenareo, but if that's the case
you are going to need a concurrent solution for communicating an error context.
