# EnergyMon Rust Bindings

The `energymon-sys` crate provides declarations and linkage for the `energymon`
C libraries.
Following the *-sys package conventions, this crate does not define
higher-level abstractions over the native library functions.

The `energymon-sys` crate only provides type bindings as defined in
`energymon.h`.

The latest `EnergyMon` C libraries can be found at
[https://github.com/energymon/energymon](https://github.com/energymon/energymon).

## Dependencies

There are no direct dependencies as this crate only provides type bindings.

## Usage
Add `energymon-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.energymon-sys]
git = "https://github.com/energymon/energymon-sys.git"
```
