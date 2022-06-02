# esp-alloc

A simple `no_std` heap allocator for RISC-V and Xtensa processors from Espressif.

Currently supports:

- ESP32
- ESP32-C3
- ESP32-S2
- ESP32-S3

**NOTE:** using this as your global allocator requires using Rust's
`nightly` release channel.

## Build Notes

In order to build this crate a valid target must be specified:

| Architecture |                                       Targets                                       |
| :----------: | :---------------------------------------------------------------------------------: |
|    RISC-V    |                            `riscv32imc-unknown-none-elf`                            |
|    Xtensa    | `xtensa-esp32-none-elf`<br/>`xtensa-esp32s2-none-elf`<br/>`xtensa-esp32s3-none-elf` |

For example:

```bash
$ cargo build --target=riscv32imc-unknown-none-elf
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
