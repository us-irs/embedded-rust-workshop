# Preparation

## Rust and `cargo`

The first thing required is the Rust compiler and the `cargo` build and dependency management
tool. The Rust installation includes the `cargo` tool, so all you need to do is install
Rust by going to the [Rust website](https://rust-lang.org/tools/install/) and following the operating
system specific instructions.

After you have done this, you can verify your installation by running:

```console
cargo version
```

Normally, you would have to also install the `thumbv7em-none-eabihf` toolchain and some other
useful tools, but this is performed automatically for you through the `rust-toolchain.toml`
file in the code directory.

## `flip-link` linker

You need to install a special linker called `flip-link` for building the software. Run
the following command in the terminal:

```console
cargo install flip-link
```

## Flasher tool `probe-rs`

Next, you need some software which allows flashing the microbit v2 via the USB interface.
We are going to use the `probe-rs` tool, which is well integrated into the Rust ecosystem:

The `probe-rs` website has [install instructions](https://probe.rs/docs/getting-started/installation/)
for various operating systems. Follow these, and then test your installation using the following
command

```console
probe-rs --version
```

## USB permission setup (Linux only)

Finally, if you are on Linux, you need to perform some steps related to `udev` to avoid permission
issues. `probe-rs` has a [page with steps you can follow](https://probe.rs/docs/getting-started/probe-setup/).

## IDE

You can use any IDE of your choice which has good [Rust Analzyer Support](https://rust-analyzer.github.io/).

If you are looking for a solid graphical IDE, [VS Code](https://code.visualstudio.com/<35;127;26M)
is an excellent choice. Make sure to install the [rust-analyzer plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) as well.

## Testing everything

Now you should have everything you need to build and flash some application to the board.
You can flash a test application now to test everything. We provide some test applications for you.

Connect the board to your computer using a Micro-USB cable. Make sure that your cable also
supports the data interface and is not power-only.

Go into the `code/app` directory.

Now, you can run the following command to build and flash a blinky application:

```console
cargo run --bin blinky
```

On the console, you should see an output like this:

```console
❯ cargo run --bin blinky
   Compiling rust-app v0.1.0 (/home/muellerr/Rust/rust-embedded-workshop/code/app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `probe-rs run --chip nRF52833_xxAA --allow-erase-all target/thumbv7em-none-eabihf/debug/blinky`
      Erasing ✔ 100% [####################]  48.00 KiB @  35.81 KiB/s (took 1s)     Finished in 3.55s
-- microbit v2 Blinky application --
```

If `probe-rs` can not detect anything, make sure that (a) the board is connected through USB, and
(b) the `udev` rules were setup properly if you are on Linux.

If everything goes right, you should see the LED in the top-left corner blinking with a frequency of 1 second.
If this is the case, congratulations, you have built and flashed an embedded Rust app and you made if through
the preparation chapter successfully!

In the [first exercise](./blinky-exercise.md), you are going to build most parts of a blinky.
