# guitar-riff: A library for reading RIFF format files

Based on the description of the format at: https://johnloomis.org/cpe102/asgn/asgn1/riff.html

## Purpose

Whilst this library **can** be used to read RIFF files, there are more mature alternatives out there (such as simply `riff`).

This was originally written in a few hours as a simple practice project to get to grips with rust.

## Limitations

`guitar-riff` only supports 32-bit RIFF files. It does not support 64-bit extensions to the format (i.e. DS64).

`riff-info` currently does not report the correct size for LIST chunks.

## `riff-info`

`riff-info` is a small utility to demonstrate the library. It accepts one argument, which is a path to a file containing a RIFF container.

You can run this using cargo from the riff-info/ directory:

```bash
cd ./riff-info/
cargo run ../1sec.wav
...
File size: 352836 bytes
Form type: WAVE
fmt : 16
data: 352800
```
