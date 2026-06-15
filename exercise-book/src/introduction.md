# Introduction

This is the exercise book accompanying the Embedded Rust Workshop.
The Workshop provides exercises for Embedded Rust which run on the
[BBC microbit v2](https://microbit.org/new-microbit/).

For this exercises, you only require the following hardware:

- The [microbit v2](https://microbit.org/new-microbit/).
- A Micro-USB cable to power the microbit while also connecting it to your computer.

This little educational board has everything we need to teach you the practical part of embedded
Rust, including:

- A 5x5 LED matrix.
- The LSM303AGR on-board motion sensor.
- A serial connection accessible by a USB-CDC device

This device has even more features and external connectivity, and you can find all
of this information of the [microbit website](https://tech.microbit.org/hardware/).

## Goals

After working through this exercises, you should have the following skills:

- Extract relevant information from datasheets and/or schematics for firmware development.
- Understand and work in embedded code using the the [`embassy`](https://embassy.dev/) asynchronous
  runtime, which includes using the `async` / `await` syntax.
- Schedule concurrent tasks using `embassy`.
- Work with some components of a provided HAL (or BSP).
- Using [`embassy-time`](https://docs.rs/embassy-time/latest/embassy_time/) to perform delays and
  periodic operations or measure elapsed time.
- Writing drivers for simple sensors

However, these exercises do not replace or provide a good theoretic foundation and it also does
not teach general Rust programming. Low-level aspects like working with registers and details about
system boot are are intentionally skipped. The next section provides some further material
recommendations to address this.

## Further Materials

If you are a beginner in Rust, it is strongly recommended to work through the [Rust book](https://doc.rust-lang.org/book/) or work
through some other method of your choice to learn the general language.

The following materials might be valuable for you as well:

- [The embassy book](https://embassy.dev/book/) provides additional information and documentation
  about the embassy asynchronous run-time.
- [The Rusty bits](https://www.youtube.com/@therustybits) provides excellent visual resources
  about various embedded Rust aspects.
- [The Ferrous Systems Rust Training slides](https://rust-training.ferrous-systems.com/latest/slides/)
  provide high-quality slides about various topics, including embedded Rust and aspects like
  system boot and peripheral access crates.
- [Microbit v2 BSP](https://github.com/lulf/microbit-bsp) provides a full BSP with drivers for
  the various board components.

## Preparation

We are going to start with the preparation of the software tools in the [next chapter](./preparation.md).
