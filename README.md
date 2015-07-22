# EnergyMon Rust Bindings

The `energymon-sys` crate provides declarations and linkage for the `energymon`
C libraries.
Following the *-sys package conventions, the `energymon-sys` crate does not
define higher-level abstractions over the native `energymon` library functions.

The latest `EnergyMon` C libraries can be found at
[https://github.com/connorimes/energymon](https://github.com/connorimes/energymon).

## Dependencies

In order to use the `energymon-sys` crate, you must have the `energymon`
libraries installed to the system.

## Usage
Add `energymon-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.energymon-sys]
git = "https://github.com/connorimes/energymon-sys.git"
```
