extern crate alloc;
extern crate console_error_panic_hook;

use alloc::vec::Vec;

use wasm_bindgen::prelude::*;

use std::cmp::Ordering;
use std::collections::BTreeSet;

use crypto_bigint::{Encoding, Zero, U256};

use rand::prelude::*;

use crate::Memory;

#[wasm_bindgen]
pub struct Secret {
    pub(crate) secret_bytes: Vec<u8>,
    pub(crate) proof_bytes: Vec<u8>,
    pub(crate) value_u256: U256,
}

impl Eq for Secret {}

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        self.value_u256 == other.value_u256
    }
}

impl PartialOrd for Secret {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Secret {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value_u256.cmp(&other.value_u256)
    }
}

#[wasm_bindgen]
pub struct Generator {
    pub(crate) mixin_bytes: Vec<u8>,
    pub(crate) price_u256: U256,
}

#[wasm_bindgen]
impl Generator {
    #[wasm_bindgen(constructor)]
    pub fn new(
        chain_u64: &Memory,
        contract_bytes: &Memory,
        receiver_bytes: &Memory,
        price_bytes: &Memory,
    ) -> Self {
        console_error_panic_hook::set_once();

        let mut mixin_bytes = vec![0u8; 128];
        mixin_bytes[0..32].clone_from_slice(&chain_u64.inner);
        mixin_bytes[32..64].clone_from_slice(&contract_bytes.inner);
        mixin_bytes[64..96].clone_from_slice(&receiver_bytes.inner);

        let price_u256 = U256::from_be_slice(&price_bytes.inner);

        Self {
            mixin_bytes,
            price_u256,
        }
    }

    #[wasm_bindgen]
    pub fn generate(&mut self) -> Generated {
        use sha3::Digest;

        let mut secret_bytes = [0u8; 32];
        let mut total_u256 = U256::ZERO;

        let mut secrets = BTreeSet::<Secret>::new();

        while total_u256 < self.price_u256 {
            rand::thread_rng().fill(&mut secret_bytes);

            let mut proof_hasher = sha3::Keccak256::new();
            proof_hasher.update(&mut secret_bytes);
            let proof_bytes = proof_hasher.finalize();

            self.mixin_bytes[96..128].copy_from_slice(&proof_bytes);

            let mut divisor_hasher = sha3::Keccak256::new();
            divisor_hasher.update(&mut self.mixin_bytes);
            let divisor_bytes = divisor_hasher.finalize();

            let divisor_u256 = U256::from_be_slice(&divisor_bytes);

            if divisor_u256.is_zero().into() {
                continue;
            }

            let value_u256 = U256::MAX.wrapping_div(&divisor_u256);

            if secrets.len() == 10 {
                let minimum = secrets.first().unwrap();

                if value_u256 < minimum.value_u256 {
                    continue;
                }

                total_u256 = total_u256.wrapping_sub(&minimum.value_u256);
                secrets.pop_first();
            }

            total_u256 = total_u256.wrapping_add(&value_u256);

            let secret = Secret {
                secret_bytes: secret_bytes.to_vec(),
                proof_bytes: proof_bytes.to_vec(),
                value_u256,
            };

            secrets.insert(secret);
        }

        Generated {
            secrets,
            total_u256,
        }
    }
}

#[wasm_bindgen]
pub struct Generated {
    pub(crate) secrets: BTreeSet<Secret>,
    pub(crate) total_u256: U256,
}

#[wasm_bindgen]
impl Generated {
    #[wasm_bindgen]
    pub fn encode_secrets(&self) -> Memory {
        let mut bytes = Vec::with_capacity(32 * self.secrets.len());

        for secret in self.secrets.iter() {
            bytes.extend_from_slice(&secret.secret_bytes);
        }

        Memory::new(bytes)
    }

    #[wasm_bindgen]
    pub fn encode_proofs(&self) -> Memory {
        let mut bytes = Vec::with_capacity(32 * self.secrets.len());

        for secret in self.secrets.iter() {
            bytes.extend_from_slice(&secret.proof_bytes);
        }

        Memory::new(bytes)
    }

    #[wasm_bindgen]
    pub fn encode_total(&self) -> Memory {
        Memory::new(self.total_u256.to_be_bytes().to_vec())
    }
}
