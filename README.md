# `stm32-template`

> A template for building applications for STM32 microcontrollers

## Dependencies

## Git Setup
To build embedded programs using this template you'll need:

- Create a .gitignore & README.md file

``` console
    touch .gitignore README.md
```

``` console
echo "# Project Title

A brief description of your project." > README.md
```

- Initialize the Git repository

``` console
git init
```

- Add the files to the staging area

``` console
git add .gitignore README.md
```

- Commit the files

``` console
git commit -m "Initial commit with .gitignore and README.md"
```

- Set up remote repo

  if you dont have Github CLI-
  ``` console
      sudo apt update
      sudo apt install gh
  ```
  then auth in..
  ``` console
  gh auth login
  ```

  ``` console
  gh repo create <repository-name> --private
  ```

- Link to remote repo and push inital commit

``` console
  git remote add origin https://github.com/<your-username>/<repository-name>.git
  git branch -M main
  git push -u origin main
```
## Install tooling
- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/cargo-generate/cargo-generate#installation).
``` console
$ cargo install cargo-generate
```

- Flash and run/debug tools:
``` console
$ cargo install probe-rs --features cli
```

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:
  
This command adds support for the following ARM Cortex-M targets:
- `thumbv6m-none-eabi`: ARM Cortex-M0 and M0+
- `thumbv7m-none-eabi`: ARM Cortex-M3
- `thumbv7em-none-eabi`: ARM Cortex-M4 and M7 (without hardware floating-point)
- `thumbv7em-none-eabihf`: ARM Cortex-M4 and M7 (with hardware floating-point)

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

To install `probe-rs`, run the following command:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

## Instantiate the template.

1. Run and enter project name
``` console
$ cargo generate --git https://github.com/burrbull/stm32-template/
 Project Name: <app>
```

2. Specify **chip product name** and answer on several other guide questions.

### if there is no template

  a. 
  ``` console 
  cargo install cargo-generate
  ```

  b.
  ``` console
  cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
  ```

  c. update Cargo.toml

  d. update memory.x


3. Your program is ready to compile:
``` console
$ cargo build --release
```

## Flash and run/debug

You can flash your firmware using one of those tools:

- `cargo flash --release` — just flash
- `cargo run --release` — flash and run using `probe-rs run` runner or `probe-run` runner (deprecated) which you can set in `.cargo/config.toml`
- `cargo embed --release` — multifunctional tool for flash and debug

You also can debug your firmware on device from VS Code with [probe-rs](https://probe.rs/docs/tools/vscode/) extention or with `probe-rs gdb` command.
You will need SVD specification for your chip for this. You can load patched SVD files [here](https://stm32-rs.github.io/stm32-rs/).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct

## blinky code

Added from <https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/delay-syst-blinky.rs>

## logging over stlink

use openocd