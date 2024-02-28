# Network Bundle

WebAssembly bundle for [Network](https://github.com/stars/hazae41/lists/network) algorithms

```bash
npm i @hazae41/network-bundle
```

[**Node Package 📦**](https://www.npmjs.com/package/@hazae41/network-bundle)

## Algorithms
- Keccak256 from RustCrypto (sha3)
- Constant-time Base16 from RustCrypto  (base16ct)
- Network generation and verification

## Features
- Reproducible building
- Pre-bundled and streamed
- Zero-copy memory slices

## Usage

### Generation

You can generate and retrieve a secret, proof and value.

```tsx
import { Memory, NetworkMixin, base16_decode_mixed, base16_encode_lower, initBundledOnce } from "@hazae41/network-bundle"

await initBundledOnce()

const chainIdBigInt = 100n
const chainIdBase16 = chainIdBigInt.toString(16).padStart(64, "0")
const chainIdMemory = base16_decode_mixed(chainIdBase16)

const contractZeroHex = "0xF1eC047cbd662607BBDE9Badd572cf0A23E1130B"
const contractBase16 = contractZeroHex.slice(2).padStart(64, "0")
const contractMemory = base16_decode_mixed(contractBase16)

const receiverZeroHex = "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4"
const receiverBase16 = receiverZeroHex.slice(2).padStart(64, "0")
const receiverMemory = base16_decode_mixed(receiverBase16)

const nonceBytes = crypto.getRandomValues(new Uint8Array(32))
const nonceMemory = new Memory(nonceBytes)
const nonceBase16 = base16_encode_lower(nonceMemory)

const minimumBigInt = 100000n
const minimumBase16 = minimumBigInt.toString(16).padStart(64, "0")
const minimumMemory = base16_decode_mixed(minimumBase16)

const mixin = new NetworkMixin(chainIdMemory, contractMemory, receiverMemory, nonceMemory)

const generated = mixin.generate(minimumMemory)

const secretMemory = generated.to_secret()
const secretBase16 = base16_encode_lower(secretMemory)
const secretZeroHex = `0x${secretBase16}`

const proofMemory = generated.to_proof()
const proofBase16 = base16_encode_lower(proofMemory)
const proofZeroHex = `0x${proofBase16}`

const valueMemory = generated.to_value()
const valueBase16 = base16_encode_lower(valueMemory)
const valueZeroHex = `0x${valueBase16}`
const valueBigInt = BigInt(valueZeroHex)

console.log(valueBigInt, secretZeroHex, proofZeroHex)
```

### Verification

#### Secret

You can verify the value of a secret.

```tsx
import { NetworkMixin, base16_decode_mixed, base16_encode_lower, initBundledOnce } from "@hazae41/network-bundle"

async function verify(secretZeroHex: string, nonceZeroHex: string): Promise<bigint> {
  await initBundledOnce()

  const chainIdBigInt = 100n
  const chainIdBase16 = chainIdBigInt.toString(16).padStart(64, "0")
  const chainIdMemory = base16_decode_mixed(chainIdBase16)

  const contractZeroHex = "0xF1eC047cbd662607BBDE9Badd572cf0A23E1130B"
  const contractBase16 = contractZeroHex.slice(2).padStart(64, "0")
  const contractMemory = base16_decode_mixed(contractBase16)

  const receiverZeroHex = "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4"
  const receiverBase16 = receiverZeroHex.slice(2).padStart(64, "0")
  const receiverMemory = base16_decode_mixed(receiverBase16)

  const nonceBase16 = nonceZeroHex.slice(2)
  const nonceMemory = base16_decode_mixed(nonceBase16)

  const mixinStruct = new NetworkMixin(chainIdMemory, contractMemory, receiverMemory, nonceMemory)

  const secretBase16 = secretZeroHex.slice(2)
  const secretMemory = base16_decode_mixed(secretBase16)

  const valueMemory = mixinStruct.verify_secret(secretMemory)
  const valueBase16 = base16_encode_lower(valueMemory)
  const valueZeroHex = `0x${valueBase16}`
  const valueBigInt = BigInt(valueZeroHex)

  return valueBigInt
}
```

#### Proof

You can zero-knowledge verify the value of a secret by using its proof.

Proofs should only account half their value because they can be spoofed for half the work.

```tsx
import { NetworkMixin, base16_decode_mixed, base16_encode_lower, initBundledOnce } from "@hazae41/network-bundle"

function preverify(proofZeroHex: string, nonceZeroHex: string): Promise<bigint> {
  await initBundledOnce()

  const chainIdBigInt = 100n
  const chainIdBase16 = chainIdBigInt.toString(16).padStart(64, "0")
  const chainIdMemory = base16_decode_mixed(chainIdBase16)

  const contractZeroHex = "0xF1eC047cbd662607BBDE9Badd572cf0A23E1130B"
  const contractBase16 = contractZeroHex.slice(2).padStart(64, "0")
  const contractMemory = base16_decode_mixed(contractBase16)

  const receiverZeroHex = "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4"
  const receiverBase16 = receiverZeroHex.slice(2).padStart(64, "0")
  const receiverMemory = base16_decode_mixed(receiverBase16)

  const nonceBase16 = nonceZeroHex.slice(2)
  const nonceMemory = base16_decode_mixed(nonceBase16)

  const mixinStruct = new NetworkMixin(chainIdMemory, contractMemory, receiverMemory, nonceMemory)

  const proofBase16 = proofZeroHex.slice(2)
  const proofMemory = base16_decode_mixed(proofBase16)

  const valueMemory = mixinStruct.verify_proof(proofMemory)
  const valueBase16 = base16_encode_lower(valueMemory)
  const valueZeroHex = `0x${valueBase16}`
  const valueBigInt = BigInt(valueZeroHex)

  return valueBigInt / 2
}
```

## Building

### Unreproducible building

You need to install [Rust](https://www.rust-lang.org/tools/install)

Then, install [wasm-pack](https://github.com/rustwasm/wasm-pack)

```bash
cargo install wasm-pack
```

Finally, do a clean install and build

```bash
npm ci && npm run build
```

### Reproducible building

You can build the exact same bytecode using Docker, just be sure you're on a `linux/amd64` host

```bash
docker compose up --build
```

Then check that all the files are the same using `git status`

```bash
git status --porcelain
```

If the output is empty then the bytecode is the same as the one I commited

### Automated checks

Each time I commit to the repository, the GitHub's CI does the following:
- Clone the repository
- Reproduce the build using `docker compose up --build`
- Throw an error if the `git status --porcelain` output is not empty

Each time I release a new version tag on GitHub, the GitHub's CI does the following:
- Clone the repository
- Do not reproduce the build, as it's already checked by the task above
- Throw an error if there is a `npm diff` between the cloned repository and the same version tag on NPM

If a version is present on NPM but not on GitHub, do not use!
