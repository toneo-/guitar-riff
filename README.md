# guitar-riff: A library for reading RIFF format files

Based on the description of the format at: https://johnloomis.org/cpe102/asgn/asgn1/riff.html

## Purpose

Whilst this library **can** be used to read RIFF files, there are more mature alternatives out there (such as simply `riff`).

This was originally written in a few hours as a simple practice project to get to grips with rust.

## Limitations

`guitar-riff` only supports 32-bit RIFF files. It does not support 64-bit extensions to the format (i.e. DS64).

## `riff-info`

`riff-info` is a small utility which outputs IDs and sizes for all chunks in a file to demonstrate the library. It accepts one argument, which is a path to a file containing a RIFF container. Output is similar to the `chunks.exe` utility (but not identical).

You can run this using cargo from the riff-info/ directory:

```bash
cd ./riff-info/
cargo run ../1sec.wav
```

```
RIFF  353140
  code: WAVE
  fmt   16
  data  352800
  LIST  134
    code: INFO
    INAM  12
    IPRD  6
    IART  6
    ICMT  22
    ICRD  6
    IGNR  12
    ITRK  10
  id3   154
```

## Possible improvements

-   Ability to modify/add chunks and then save them. Allows writing files.
-   Ability to quickly enumerate all top-level chunks in a file, without reading all their data into memory. For fast cataloguing purposes.
-   Tests for reading chunks within LIST chunks. I got lazy and didn't want to write these.
