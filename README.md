This application malfunctions when compiled for AVR and run on an Arduino Uno.

It logs the following to the serial port:

```
Hello, world!
wat 248 57351 7936
debug buf [0, 57568, 7967] mapped
debug buf [63488, 2016, 31] copied
debug buf [0, 57568, 7967] in-loop
debug buf [248, 57351, 7936] enumerate
debug as_byte_slice() [248, 0, 7, 224, 0, 31]
```

If compiled with less aggressive optimization (`opt-level = "s"`) it emits the correct

```
Hello, world!
wat 248 57351 7936
debug buf [248, 57351, 7936] mapped
debug buf [63488, 2016, 31] copied
debug buf [248, 57351, 7936] in-loop
debug buf [248, 57351, 7936] enumerate
debug as_byte_slice() [248, 0, 7, 224, 0, 31]
```

This malfunction interferes with the SPI transfers performed by the st7789 display driver (results in incorrect colors).

The app that led to the discovery of this bug malfunctions even with `opt-level="s"`.

In addition to nightly-2022-05-10 (from rust-toolchain.toml), whichever nightly-x86_64-unknown-linux-gnu I happen to have installed also malfunctions.
