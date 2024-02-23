extern crate alloc;

use alloc::vec::Vec;

use wasm_bindgen::prelude::*;

use std::cmp::Ordering;
use std::collections::BTreeSet;

use crypto_bigint::{Encoding, Zero, U256};

use rand::prelude::*;

use crate::Memory;

#[wasm_bindgen]
pub struct NetworkSecret {
    pub(crate) secret_bytes: Vec<u8>,
    pub(crate) proof_bytes: Vec<u8>,
    pub(crate) value_u256: U256,
}

impl Eq for NetworkSecret {}

impl PartialEq for NetworkSecret {
    fn eq(&self, other: &Self) -> bool {
        self.secret_bytes == other.secret_bytes
    }
}

impl PartialOrd for NetworkSecret {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NetworkSecret {
    fn cmp(&self, other: &Self) -> Ordering {
        self.secret_bytes.cmp(&other.secret_bytes)
    }
}

#[wasm_bindgen]
pub struct NetworkMixin {
    pub(crate) mixin_bytes: Vec<u8>,
}

#[wasm_bindgen]
impl NetworkMixin {
    #[wasm_bindgen(constructor)]
    pub fn new(
        chain_u64: &Memory,
        contract_bytes: &Memory,
        receiver_bytes: &Memory,
        nonce_bytes: &Memory,
    ) -> Self {
        let mut mixin_bytes = vec![0u8; 160];
        mixin_bytes[0..32].clone_from_slice(&chain_u64.inner);
        mixin_bytes[32..64].clone_from_slice(&contract_bytes.inner);
        mixin_bytes[64..96].clone_from_slice(&receiver_bytes.inner);
        mixin_bytes[96..128].clone_from_slice(&nonce_bytes.inner);

        Self { mixin_bytes }
    }

    #[wasm_bindgen]
    pub fn generate(&mut self, price_bytes: &Memory) -> NetworkGenerated {
        use sha3::Digest;

        let price_u256 = U256::from_be_slice(&price_bytes.inner);

        let mut secret_bytes = [0u8; 32];
        let mut total_u256 = U256::ZERO;

        let mut secrets_set = BTreeSet::<NetworkSecret>::new();

        while total_u256 < price_u256 {
            rand::thread_rng().fill(&mut secret_bytes);

            let mut proof_hasher = sha3::Keccak256::new();
            proof_hasher.update(&mut secret_bytes);
            let proof_bytes = proof_hasher.finalize();

            self.mixin_bytes[128..160].copy_from_slice(&proof_bytes);

            let mut divisor_hasher = sha3::Keccak256::new();
            divisor_hasher.update(&mut self.mixin_bytes);
            let divisor_bytes = divisor_hasher.finalize();

            let divisor_u256 = U256::from_be_slice(&divisor_bytes);

            if divisor_u256.is_zero().into() {
                continue;
            }

            let value_u256 = U256::MAX.wrapping_div(&divisor_u256);

            if secrets_set.len() == 10 {
                let minimum = secrets_set.first().unwrap();

                if value_u256 < minimum.value_u256 {
                    continue;
                }

                total_u256 = total_u256.wrapping_sub(&minimum.value_u256);

                secrets_set.pop_first();
            }

            total_u256 = total_u256.wrapping_add(&value_u256);

            let secret = NetworkSecret {
                secret_bytes: secret_bytes.to_vec(),
                proof_bytes: proof_bytes.to_vec(),
                value_u256,
            };

            secrets_set.insert(secret);
        }

        NetworkGenerated {
            secrets_set,
            total_u256,
        }
    }

    #[wasm_bindgen]
    pub fn verify_secrets(&mut self, secrets_bytes: &Memory) -> Memory {
        use sha3::Digest;

        let secrets_chunks = secrets_bytes.inner.chunks_exact(32);

        let mut total_u256 = U256::ZERO;

        for secret_bytes in secrets_chunks {
            let mut proof_hasher = sha3::Keccak256::new();
            proof_hasher.update(secret_bytes);
            let proof_bytes = proof_hasher.finalize();

            self.mixin_bytes[128..160].copy_from_slice(&proof_bytes);

            let mut divisor_hasher = sha3::Keccak256::new();
            divisor_hasher.update(&mut self.mixin_bytes);
            let divisor_bytes = divisor_hasher.finalize();

            let divisor_u256 = U256::from_be_slice(&divisor_bytes);

            if divisor_u256.is_zero().into() {
                continue;
            }

            let value_u256 = U256::MAX.wrapping_div(&divisor_u256);

            total_u256 = total_u256.wrapping_add(&value_u256);
        }

        Memory::new(total_u256.to_be_bytes().to_vec())
    }

    #[wasm_bindgen]
    pub fn verify_proofs(&mut self, proofs_bytes: &Memory) -> Memory {
        use sha3::Digest;

        let proofs_chunks = proofs_bytes.inner.chunks_exact(32);

        let mut total_u256 = U256::ZERO;

        for proof_bytes in proofs_chunks {
            self.mixin_bytes[128..160].copy_from_slice(proof_bytes);

            let mut divisor_hasher = sha3::Keccak256::new();
            divisor_hasher.update(&mut self.mixin_bytes);
            let divisor_bytes = divisor_hasher.finalize();

            let divisor_u256 = U256::from_be_slice(&divisor_bytes);

            if divisor_u256.is_zero().into() {
                continue;
            }

            let value_u256 = U256::MAX.wrapping_div(&divisor_u256);

            total_u256 = total_u256.wrapping_add(&value_u256);
        }

        Memory::new(total_u256.to_be_bytes().to_vec())
    }
}

#[wasm_bindgen]
pub struct NetworkGenerated {
    pub(crate) secrets_set: BTreeSet<NetworkSecret>,
    pub(crate) total_u256: U256,
}

#[wasm_bindgen]
impl NetworkGenerated {
    #[wasm_bindgen]
    pub fn encode_secrets(&self) -> Memory {
        let mut bytes = Vec::with_capacity(32 * self.secrets_set.len());

        for secret in self.secrets_set.iter() {
            bytes.extend_from_slice(&secret.secret_bytes);
        }

        Memory::new(bytes)
    }

    #[wasm_bindgen]
    pub fn encode_proofs(&self) -> Memory {
        let mut bytes = Vec::with_capacity(32 * self.secrets_set.len());

        for secret in self.secrets_set.iter() {
            bytes.extend_from_slice(&secret.proof_bytes);
        }

        Memory::new(bytes)
    }

    #[wasm_bindgen]
    pub fn encode_total(&self) -> Memory {
        Memory::new(self.total_u256.to_be_bytes().to_vec())
    }
}
