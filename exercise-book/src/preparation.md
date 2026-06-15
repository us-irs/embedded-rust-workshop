# Preparation

## Windows Terminal

It is recommended to use a terminal for some parts of this workshop. There will be a lot
of steps and instructions which show terminal commands. On Windows, you should install
a proper terminal unless you plan to use something like [WSL](https://learn.microsoft.com/en-us/windows/wsl/install).

For example, you can [install the PowerShell](https://learn.microsoft.com/en-us/powershell/scripting/install/install-powershell-on-windows?view=powershell-7.5#winget).

## Cloning the project with `git`

It is strongly recommended to clone with project with `git`. If you have never worked with git
before, you should work through a tutorial online first, for example [this interactive one](https://learngitbranching.js.org/).
Alternatively, have a look at the [learning resources](https://git-scm.com/learn).
If you are a `git` beginner, you can install it using [the website instructions](https://git-scm.com/install/).

The project is hosted on [GitHub](https://github.com/us-irs/embedded-rust-workshop) publicly.
It is also hosted on the [IRS GitLab](https://git.irs.uni-stuttgart.de/irs/embedded-rust-workshop)
which you can access if you are an IRS employee.

You can clone this workshop by running the following git command:

```console
git clone https://github.com/us-irs/embedded-rust-workshop.git
```

or with SSH:

```console
git clone git@github.com:us-irs/embedded-rust-workshop.git
```

If you are an IRS employee and have access to the IRS GitLab:

```console
git clone https://git.irs.uni-stuttgart.de/irs/embedded-rust-workshop.git
```

or with SSH:

```console
git clone git@git.irs.uni-stuttgart.de:irs/embedded-rust-workshop.git
```

Have a look at the README of the repository. It explains the diretory structure. The
most relevant part of the repository for you is the `microbit-code/exercises` folder.

Now that you have cloned the project, you might have to set up some software.

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

You can use any IDE of your choice which has good [Rust Analyzer Support](https://rust-analyzer.github.io/).

If you are looking for a solid graphical IDE, [VS Code](https://code.visualstudio.com/<35;127;26M)
is an excellent choice. Make sure to install the [rust-analyzer plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) as well.

## Testing everything

If you are preparing everything for the workshop, and you do not have access to the hardware yet,
you are done! You can perform this step once you have access to the hardware in the workshop.

Now you should have everything you need to build and flash some application to the board.
You can flash a test application now to test everything. We provide some test applications for you.

Connect the board to your computer using a Micro-USB cable. Make sure that your cable also
supports the data interface and is not power-only.

Navigate into the `microbit-apps` directory.

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
