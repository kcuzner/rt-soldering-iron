# Firmware for the RT Soldering Iron, written in rust

## Why

Rust is a system-level language which should allow us to retain the
low-levelness of C while adding the following:

- Memory safety
- Some semblance of enforced data safety
- All the awesomeness that comes with pattern matching and other language
  features which would be really handy.

This is not without cost, however. I believe that generally this firmware will
have a larger footprint than something equivalent in C (assuming that we don't
use FreeRTOS of course, that thing is huge).

### Data safety

Since there are lot of stateful things in the microcontroller, the scoped
singleton pattern is used extensively. As the state of the various peripherals
change, the singletons will be consumed and transform themselves into different
objects which embody some different behavior.

### Memory Allocations

Since this is a `no_std` project, there is no heap or allocator. All variables
must either be `static` or allocated on the stack. As such, this project does
make some use of the `'static` lifetime and `static mut`.

## Building

This uses a `Makefile.toml` to execute various `arm-none-eabi` commands along
with building the project to facilitate convenient embedded development. In
order to build the project conveniently, the following must be installed:

 - Nightly Rust
 - The thumbv6m target: `$ rustup target add thumbv6m-none-eabi`
 - [cargo-make](https://github.com/sagiegurari/cargo-make)
 - The `arm-none-eabi` toolchain and binutils
 - Python
 - Openocd

There is a python script in the `./scripts` directory which handles various
useful binary operations (dumping, sizing, etc) and also handles flashing the
microcontroller using openocd. This is quite un-rustlike (having a python script
as part of a build process), but it was the fastest way for me to get going.

To build the project binary in release mode, execute the following:

```
$ cargo make
```

To install the project on a microcontroller attached to an ST-Link v2, execute
the following:

```
$ cargo make install-embedded-release
```

## Debugging

To start openocd, navigate to `./scripts` and simply run `openocd`. It will
start in server mode. In another window, navigate to the target directory (where
the `rt-solering-iron` binary is located) and execute:

```
$ arm-none-eabi-gdb -ex "target remote localhost:3333" rt-soldering-iron
```

As usual with gdb, use the `monitor` command to send commands to openocd, such
as `monitor halt` and `monitor resume`.

