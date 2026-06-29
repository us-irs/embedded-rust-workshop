# Terminology and Glossary

You can look up some terms in this terminology chapter if you have never heard them before or need
a brief explanation.

- Peripheral: Dedicated hardware unit. On microcontrollers, this can be something like a UART, SPI
  or timer hardware block.
- Flash: Non-volatile memory which you can use to store your code and constants.
- Stack: Local memory that your program uses as it executes functions.
- Heap: Free memory that can be allocated in blocks. Oftentimes not available on microcontrollers.
- Static: Usually refers to the lifetime of a variable. A static variable is valid for the whole
  program duration.
- RAM: Volatile memory used to store your stack, heap and static variables.
- Processor: Hardware block which executes your code.
- MCU or Microcontroller: Integrates the processor, peripherals, flash, RAM and is usually
  placed on a printed circuit board as part of an embedded system.
- Firmware: Software which interacts closely with hardware. Usually refers to the finished software
  product on your MCU which runs the embedded system.
- PCB - Printed Circuit Board: This is usually an electronic circuit integrated on a sandwich
  structure. Often, chips, sensors, and other components will be soldered on top of the PCB.
- UART - Universal Asynchronous Receiver-Transmitter: Asynchronous serial communication interface
  that only requires two physical pins, one for transmission and one for reception.
- DMA - Direct Memory Access: A hardware subsystem can access the memory of a system directly
  without CPU intervention.
- CPU - Central Processing Unit: The core hardware block of a computer or microcontroller that
  executes instructions.
- HAL - Hardware Abstraction Layer: High level library providing drivers for the hardware blocks
  on a microcontroller.
- I2C - Inter-Integrated Circuit: Communication bus commonly used on embedded systems.
- TWI - Two-Wire Interface: Another name for I2C that vendors sometimes use.
- GPIO - General Purpose Input/Output: A digital signal pin on a microcontroller that can be
  configured as an input to read a digital level or as an output to set a digital level.
- SPI - Serial Peripheral Interface: A synchronous serial communication bus commonly used on
  embedded systems, typically requiring four signals: clock, chip select, MOSI and MISO.
- RTT - Real-Time Transfer: A SEGGER protocol for transferring data between a host computer and
  an embedded target, commonly used for logging output during development.
- BSP - Board Support Package: A library that provides drivers and abstractions specific to a
  particular hardware board, building on top of a HAL.
- IPC - Inter-Process Communication: Mechanisms that allow concurrent tasks or processes to
  exchange data and synchronize with each other.
