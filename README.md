# Netbundle

WebAssembly bundle for Network algorithms

```bash
npm i @hazae41/netbundle
```

[**Node Package 📦**](https://www.npmjs.com/package/@hazae41/netbundle)

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

```tsx
import { NetworkMixin, base16_decode_mixed, base16_encode_lower, initBundledOnce } from "@hazae41/netbundle"

await initBundledOnce()

const chainIdBigInt = 1n
const chainIdBase16 = chainIdBigInt.toString(16).padStart(64, "0")
const chainIdMemory = base16_decode_mixed(chainIdBase16)

const contractZeroHex = "0xB57ee0797C3fc0205714a577c02F7205bB89dF30"
const contractBase16 = contractZeroHex.slice(2).padStart(64, "0")
const contractMemory = base16_decode_mixed(contractBase16)

const receiverZeroHex = "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4"
const receiverBase16 = receiverZeroHex.slice(2).padStart(64, "0")
const receiverMemory = base16_decode_mixed(receiverBase16)

const mixinStruct = new NetworkMixin(chainIdMemory, contractMemory, receiverMemory)

const priceBigInt = 10000n
const priceBase16 = priceBigInt.toString(16).padStart(64, "0")
const priceMemory = base16_decode_mixed(priceBase16)

const generatedStruct = mixinStruct.generate(priceMemory)

const secretsMemory = generatedStruct.encode_secrets()
const secretsBase16 = base16_encode_lower(secretsMemory)
const secretsZeroHex = `0x${secretsBase16}`

const proofsMemory = generatedStruct.encode_proofs()
const proofsBase16 = base16_encode_lower(proofsMemory)
const proofsZeroHex = `0x${proofsBase16}`

const totalMemory = generatedStruct.encode_total()
const totalBase16 = base16_encode_lower(totalMemory)
const totalZeroHex = `0x${totalBase16}`
const totalBigInt = BigInt(totalZeroHex)

console.log(totalBigInt, secretsZeroHex, proofsZeroHex)
```

### Verification

#### Secrets

```tsx
import { NetworkMixin, base16_decode_mixed, base16_encode_lower, initBundledOnce } from "@hazae41/netbundle"

await initBundledOnce()

const chainIdBigInt = 1n
const chainIdBase16 = chainIdBigInt.toString(16).padStart(64, "0")
const chainIdMemory = base16_decode_mixed(chainIdBase16)

const contractZeroHex = "0xB57ee0797C3fc0205714a577c02F7205bB89dF30"
const contractBase16 = contractZeroHex.slice(2).padStart(64, "0")
const contractMemory = base16_decode_mixed(contractBase16)

const receiverZeroHex = "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4"
const receiverBase16 = receiverZeroHex.slice(2).padStart(64, "0")
const receiverMemory = base16_decode_mixed(receiverBase16)

const mixinStruct = new NetworkMixin(chainIdMemory, contractMemory, receiverMemory)

const secretsZeroHex = "0x..."
const secretsBase16 = secretsZeroHex.slice(2)
const secretsMemory = base16_decode_mixed(secretsBase16)

const totalMemory = mixingStruct.verify_secrets(secretsMemory)
const totalBase16 = base16_encode_lower(totalMemory)
const totalZeroHex = `0x${totalBase16}`
const totalBigInt = BigInt(totalZeroHex)

console.log(totalBigInt)
```

#### Proofs

```tsx
import { NetworkMixin, base16_decode_mixed, base16_encode_lower, initBundledOnce } from "@hazae41/netbundle"

await initBundledOnce()

const chainIdBigInt = 1n
const chainIdBase16 = chainIdBigInt.toString(16).padStart(64, "0")
const chainIdMemory = base16_decode_mixed(chainIdBase16)

const contractZeroHex = "0xB57ee0797C3fc0205714a577c02F7205bB89dF30"
const contractBase16 = contractZeroHex.slice(2).padStart(64, "0")
const contractMemory = base16_decode_mixed(contractBase16)

const receiverZeroHex = "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4"
const receiverBase16 = receiverZeroHex.slice(2).padStart(64, "0")
const receiverMemory = base16_decode_mixed(receiverBase16)

const mixinStruct = new NetworkMixin(chainIdMemory, contractMemory, receiverMemory)

const proofsZeroHex = "0x..."
const proofsBase16 = proofsZeroHex.slice(2)
const proofsMemory = base16_decode_mixed(proofsBase16)

const totalMemory = mixingStruct.verify_proofs(proofsMemory)
const totalBase16 = base16_encode_lower(totalMemory)
const totalZeroHex = `0x${totalBase16}`
const totalBigInt = BigInt(totalZeroHex)

console.log(totalBigInt)
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
