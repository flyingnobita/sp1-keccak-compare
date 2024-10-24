// This program compares the number of RISC-V cycles of keccak256 with and without precompile.

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::hex;

// precompile for keccak256
use tiny_keccak::{Hasher, Keccak};

// pure rust implementation of keccak256
use sha3::{Digest, Keccak256};

/// Simple interface to the [`keccak256`] hash function.
fn keccak256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}

fn test_keccak_with_precompile() {
    let input = [1u8; 32];
    let expected_output = hex!("cebc8882fecbec7fb80d2cf4b312bec018884c2d66667c67a90508214bd8bafc");

    let output = keccak256(input);
    assert_eq!(output, expected_output);
}

fn test_keccak_without_precompile() {
    let input = [1u8; 32];
    let expected_output = hex!("cebc8882fecbec7fb80d2cf4b312bec018884c2d66667c67a90508214bd8bafc");

    let mut sha3_10_8 = Keccak256::new();
    sha3_10_8.update(input);
    let output_sha3_10_8: [u8; 32] = sha3_10_8.finalize().into();
    assert_eq!(output_sha3_10_8, expected_output)
}

pub fn main() {
    // test_keccak_with_precompile(); // Number of cycles: 5775

    test_keccak_without_precompile(); // Number of cycles: 21312
}
