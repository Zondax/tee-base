# RuZTEE Framework

This repository's purpose is to allow the easy build of any OPTEE rust application using the template we provide.

## How to use

After having [built qemu with optee support](https://github.com/sccommunity/rust-optee-trustzone-sdk/wiki/Getting-started-with-OPTEE-for-QEMU-ARMv8), a couple of environment variables are still needed.

Depending on what kind of ARM you have setup QEMU for, either set `QEMU_V8` or `QEMU_V7` environment variables.
Set `SHARED_FOLDER` to the folder that you want to mount in QEMU to share files between your system and the VM.

If the `OPTEE` variable is provided (pointing to an optee-qemu installation) it's possible to use `make qemu` to build qemu (only available for `QEMU_V8`, v7 not tested yet).

Set `REPO` to the url to be cloned by git and used as the project's source.

Then proceed to `make deps` to install the required rust tooling and clone your project's source.

To run, simple `make run` (or `run-debug` for added debug arguments)

After logging in the machine via terminal the folder should match the one you set as `SHARED_FOLDER`, if not you might need to `make qemu-clean qemu` to rebuild the qemu image

## Crates

Some crates are presend in this repo under the `crates` folder.
These crates provide wrappers around the OPTEE api.

- `zondee-utee` is used by TAs. It provides, among other things, `TEEAllocator`, `TEELogger`, `TEERng`.
- `zondee-teec` is used by Host applications. It provides things like `Operation` to send to the associated TA.
- `zondee-macros` has procedural macros used in the above crates
- `zondee` contains definitions shared across the above crates, suc has `Uuid`
