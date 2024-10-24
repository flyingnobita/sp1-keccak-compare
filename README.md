# SP1 Keccak Precompile Comparison

This program compare the performance of the keccak-256 hash function when using precompile versus a rust native implementation.

|            | Without Precompile | With Precompile | Savings With Precompile |
| ---------- | ------------------ | --------------- | ----------------------- |
| Cycles     | 21312              | 5775            | 73%                     |
| Gas        | 44581              | 26143           | 41%                     |
| khz        | 3.11               | 0.67            | 78%                     |
| Proof Size | 2656912            | 5466801         | -106%                   |

## Running the Project

### Update the Program

Modify `program/src/main::main` to choose whether to run with or without the keccak precompile.

### Build the Program

To build the program, run the following command:

```sh
cd program
cargo prove build
```

### Execute the Program

To execute the program and see the number of cycles used:

```sh
cd script
cargo run --release -- --execute
```

### Generate a Core Proof

To generate a proof and see the number of cycles, gas, khz and proof size:

```sh
cd script
cargo run --release -- --prove
```
