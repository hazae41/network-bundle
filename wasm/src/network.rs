extern crate alloc;

use alloc::vec::Vec;

use wasm_bindgen::prelude::*;

use crypto_bigint::{Encoding, Zero, U256};

use rand::prelude::*;

use crate::Memory;

#[wasm_bindgen]
pub struct NetworkSecret {
    pub(crate) secret_bytes: Vec<u8>,
    pub(crate) proof_bytes: Vec<u8>,
    pub(crate) value_u256: U256,
}

#[wasm_bindgen]
impl NetworkSecret {
    #[wasm_bindgen]
    pub fn to_secret(&self) -> Memory {
        Memory::new(self.secret_bytes.clone())
    }

    #[wasm_bindgen]
    pub fn to_proof(&self) -> Memory {
        Memory::new(self.proof_bytes.clone())
    }

    #[wasm_bindgen]
    pub fn to_value(&self) -> Memory {
        Memory::new(self.value_u256.to_be_bytes().to_vec())
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
        chain_memory: &Memory,
        contract_memory: &Memory,
        receiver_nonce: &Memory,
        nonce_memory: &Memory,
    ) -> Self {
        let mut mixin_bytes = vec![0u8; 160];
        mixin_bytes[0..32].clone_from_slice(&chain_memory.inner);
        mixin_bytes[32..64].clone_from_slice(&contract_memory.inner);
        mixin_bytes[64..96].clone_from_slice(&receiver_nonce.inner);
        mixin_bytes[96..128].clone_from_slice(&nonce_memory.inner);

        Self { mixin_bytes }
    }

    #[wasm_bindgen]
    pub fn generate(&mut self, minimum_memory: &Memory) -> NetworkSecret {
        use sha3::Digest;

        let minimum_u256 = U256::from_be_slice(&minimum_memory.inner);

        let mut secret_bytes = [0u8; 32];

        loop {
            rand::thread_rng().fill(&mut secret_bytes);

            let mut proof_hasher = sha3::Keccak256::new();
            proof_hasher.update(&secret_bytes);
            let proof_bytes = proof_hasher.finalize();

            self.mixin_bytes[128..160].copy_from_slice(&proof_bytes);

            let mut divisor_hasher = sha3::Keccak256::new();
            divisor_hasher.update(&self.mixin_bytes);
            let divisor_bytes = divisor_hasher.finalize();

            let divisor_u256 = U256::from_be_slice(&divisor_bytes);

            if divisor_u256.is_zero().into() {
                continue;
            }

            let value_u256 = U256::MAX.wrapping_div(&divisor_u256);

            if value_u256 < minimum_u256 {
                continue;
            }

            return NetworkSecret {
                secret_bytes: secret_bytes.to_vec(),
                proof_bytes: proof_bytes.to_vec(),
                value_u256,
            };
        }
    }

    #[wasm_bindgen]
    pub fn verify_secret(&mut self, secret_memory: &Memory) -> Memory {
        use sha3::Digest;

        let mut proof_hasher = sha3::Keccak256::new();
        proof_hasher.update(&secret_memory.inner);
        let proof_bytes = proof_hasher.finalize();

        self.mixin_bytes[128..160].copy_from_slice(&proof_bytes);

        let mut divisor_hasher = sha3::Keccak256::new();
        divisor_hasher.update(&mut self.mixin_bytes);
        let divisor_bytes = divisor_hasher.finalize();

        let divisor_u256 = U256::from_be_slice(&divisor_bytes);

        if divisor_u256.is_zero().into() {
            return Memory::new(U256::ZERO.to_be_bytes().to_vec());
        }

        let value_u256 = U256::MAX.wrapping_div(&divisor_u256);

        Memory::new(value_u256.to_be_bytes().to_vec())
    }

    #[wasm_bindgen]
    pub fn verify_proof(&mut self, proof_memory: &Memory) -> Memory {
        use sha3::Digest;

        self.mixin_bytes[128..160].copy_from_slice(&proof_memory.inner);

        let mut divisor_hasher = sha3::Keccak256::new();
        divisor_hasher.update(&mut self.mixin_bytes);
        let divisor_bytes = divisor_hasher.finalize();

        let divisor_u256 = U256::from_be_slice(&divisor_bytes);

        if divisor_u256.is_zero().into() {
            return Memory::new(U256::ZERO.to_be_bytes().to_vec());
        }

        let value_u256 = U256::MAX.wrapping_div(&divisor_u256);

        Memory::new(value_u256.to_be_bytes().to_vec())
    }
}
