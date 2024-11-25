# Wearable Computing

## Requirements

- The standard Rust tooling (cargo, rustup) which can be installed from https://rustup.rs/
- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)
- A [`probe-rs` installation](https://probe.rs/docs/getting-started/installation/)
- A [OpenOCD installation](https://openocd.org/pages/getting-openocd.html)
- A second Raspberry Pi Pico board as a debug probe

### Preparing the Raspberry Pi Pico Debug Probe

You can use a second Pico as your debugger.

Download one of these firmware files:

- [picoprobe.uf2](https://github.com/raspberrypi/picoprobe/releases/download/picoprobe-cmsis-v1.02/picoprobe.uf2) -
  Official raspberrypi probe firmware supporting CMSIS-DAP. ([Source](https://github.com/raspberrypi/picoprobe))
- [raspberry_pi_pico-DapperMime.uf2](https://github.com/majbthrd/DapperMime/releases/download/20210225/raspberry_pi_pico-DapperMime.uf2) -
  Based upon an older version of the CMSIS-DAP sources. ([Source](https://github.com/majbthrd/DapperMime))
- [rust-dap-pico-ramexec-setclock.uf2](https://raw.githubusercontent.com/9names/binary-bits/main/rust-dap-pico-ramexec-setclock.uf2) -
  If you have good wiring between your Pico's, this firmware will give faster
  programming. (Inofficial build by [@9names](https://github.com/9names/).) ([Source](https://github.com/ciniml/rust-dap))

Then:

1. Put the Pico into USB Mass Storage Mode by holding the BOOTSEL button while connecting it to your computer with a USB cable
2. Open the drive RPI-RP2 when prompted
3. Copy the uf2 firmware file from Downloads into RPI-RP2
4. Connect the debug pins of your CMSIS-DAP Pico to the target one
   - Connect GP2 on the Probe to SWCLK on the Target
   - Connect GP3 on the Probe to SWDIO on the Target
   - Connect a ground line from the CMSIS-DAP Probe to the Target too

## Installation of development dependencies

```bash
rustup target install thumbv6m-none-eabi
cargo install flip-link
cargo install --locked probe-rs-tools
```

## Running the code

For a debug build

```bash
cargo run
```

For a release build

```bash
cargo run --release
```
