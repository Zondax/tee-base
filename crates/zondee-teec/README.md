# Optee TEEC

Raw and safe wrappers around the native Optee host calls.

There are two main modules, `framework` and `wrapper`. Each was its own responsability, `wrapper` is all about the necessary machinery to work with TEE targets while `framework` is a high-level abstraction that tries to hide the low-level details of `wrapper`.