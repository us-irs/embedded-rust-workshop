[![book](https://img.shields.io/badge/workshop-book-darkgreen?style=flat)](https://robamu.github.io/embedded-workshop/book/)

Embedded Rust Workshop
==========

This repository contains all components of the Embedded Rust Workshop.

This includes:

- [`firmware` workspace](./firmware/) contains all source code for the micro:bit v2 target hardware.
- [`firmware/exercises`](./firmware/exercises/) contains all example source code and exercises which
  run on the micro:bit v2 target hardware
- [`firmware/apps`](./firmware/apps/) contains sample apps which you can also use to test the board
  setup.
- [`host` workspace](./host) contains all source code which runs on the host (PC) or libraries
  shared between firmware and host software.
- [`exercise-book`](./exercise-book) contains the accompanying exercise book. The book is also
  hosted [online](https://robamu.github.io/embedded-workshop/book/).
- [`slides`](./slides) contains introduction slides about this workshop and embedded Rust.
- [`host-client`](./host-client/) contains a host client used to communicate with the micro:bit v2
  via a serial interface.

