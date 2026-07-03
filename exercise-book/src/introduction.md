# Introduction

This is the exercise book accompanying the Embedded Rust Workshop.
The Workshop provides exercises for Embedded Rust which run on the
[BBC micro:bit v2](https://microbit.org/new-microbit/).

For these exercises, you only require the following hardware:

- The [micro:bit v2](https://microbit.org/new-microbit/).
- A Micro-USB cable to power the microbit while also connecting it to your computer.

This little educational board has everything we need to learn the practical side of embedded
Rust, including:

- A 5x5 LED matrix.
- The LSM303AGR on-board motion sensor.
- A serial connection accessible by a USB-CDC device

This device has even more features and external connectivity, and you can find all
of this information on the [micro:bit website](https://tech.microbit.org/hardware/).

## Goals

After working through these exercises, you should have the following skills:

- Extract relevant information from datasheets and/or schematics for firmware development.
- Understand and work in embedded Rust code using the [`embassy`](https://embassy.dev/) asynchronous
  runtime, which includes using the `async` / `await` syntax.
- Work with some components of a provided hardware abstraction layer (HAL) or board support
  package (BSP).
- Learn the bare basics about embedded peripherals like GPIO, I2C and UART.
- Schedule concurrent tasks using `embassy`.
- Using [`embassy-time`](https://docs.rs/embassy-time/latest/embassy_time/) to perform delays and
  periodic operations or measure elapsed time.
- Writing drivers for simple sensors
- Exchanging telecommands and telemetry with an embedded firmware application using a client
  running on your computer.

## Non-Goals

These exercises do not replace or provide a good theoretical foundation and also do
not teach general Rust programming systematically. Furthermore, they do not replace a good
foundation about embedded systems. We will still try to condense and teach the required embedded
knowledge for completing the exercise in the materials. Low-level aspects like working with
registers and details about system boot are intentionally skipped.

If you are a beginner in Rust and/or embedded systems, you can still work through this workshop
but a lot of sections might be harder and/or confusing.

The next section provides some further material recommendations to address this.

## Further Materials

If you are a beginner in Rust, it is strongly recommended to work through the [Rust book](https://doc.rust-lang.org/book/)
or work through some other method of your choice to learn the general language.

The following materials might be valuable for you as well:

- [The embassy book](https://embassy.dev/book/) provides additional information and documentation
  about the embassy asynchronous runtime.
- [The Rusty bits](https://www.youtube.com/@therustybits) provides excellent visual resources
  about various embedded Rust aspects.
- [The Ferrous Systems Rust Training slides](https://rust-training.ferrous-systems.com/latest/slides/)
  provide high-quality slides about various topics, including embedded Rust and aspects like
  system boot and peripheral access crates.
- [micro:bit BSP](https://github.com/lulf/microbit-bsp) provides a full BSP with drivers for
  the various board components.
- [Basics of UART communication](https://www.circuitbasics.com/basics-uart-communication/)
- [Basics of SPI communication](https://www.circuitbasics.com/basics-of-the-spi-communication-protocol/)
- [Basics of the I2C communication](https://www.circuitbasics.com/basics-of-the-i2c-communication-protocol/)

## Preparation

We are going to start with the preparation of the software tools in the [next chapter](./preparation.md).
