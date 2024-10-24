# SP1 Keccak Precompile Comparison

This program compares the performance of the `keccak-256` hash function when using precompile versus a rust native implementation.

Here is the result of running `keccak-256` 100 times:

|                           | Without Precompile | With Precompile | Savings With Precompile |
| ------------------------- | -----------------: | --------------: | ----------------------: |
| Cycles                    |            1676023 |          122717 |                     93% |
| Gas                       |            1951203 |          195375 |                     90% |
| End-to-End Prove Time (s) |              94.10 |           14.79 |                     84% |
| khz                       |              17.56 |            8.30 |                     53% |
| Proof Size (bytes)\*      |            7123300 |         5466801 |                     23% |

\* Proof Size with precompile stays constant at 5466801 bytes, regardless of the number of iterations.

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

To generate a proof and see the number of cycles, gas, proof time, khz and proof size:

```sh
cd script
cargo run --release -- --prove
```
