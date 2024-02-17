import { NetworkMixin, base16_decode_mixed, base16_encode_lower, initBundledOnce } from "@hazae41/network-bundle"

await initBundledOnce()

/**
 * Chain ID
 */
const chainIdBigInt = 1n
const chainIdBase16 = chainIdBigInt.toString(16).padStart(64, "0")
const chainIdMemory = base16_decode_mixed(chainIdBase16)

/**
 * Contract address
 */
const contractZeroHex = "0xB57ee0797C3fc0205714a577c02F7205bB89dF30"
const contractBase16 = contractZeroHex.slice(2).padStart(64, "0")
const contractMemory = base16_decode_mixed(contractBase16)

/**
 * Receiver address
 */
const receiverZeroHex = "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4"
const receiverBase16 = receiverZeroHex.slice(2).padStart(64, "0")
const receiverMemory = base16_decode_mixed(receiverBase16)

/**
 * Price
 */
const priceBigInt = 10000n
const priceBase16 = priceBigInt.toString(16).padStart(64, "0")
const priceMemory = base16_decode_mixed(priceBase16)

const mixin = new NetworkMixin(chainIdMemory, contractMemory, receiverMemory)

const start = performance.now()
const generated = mixin.generate(priceMemory)
const end = performance.now()

const secretsMemory = generated.encode_secrets()
const secretsBase16 = base16_encode_lower(secretsMemory)

const proofsMemory = generated.encode_proofs()
const proofsBase16 = base16_encode_lower(proofsMemory)

const verifiedMemory = mixin.verify_secrets(secretsMemory)

const totalBase16 = base16_encode_lower(generated.encode_total())
const totalBigInt = BigInt("0x" + totalBase16)

for (let i = 0; i < secretsBase16.length; i += 64)
  console.log(secretsBase16.slice(i, i + 64), proofsBase16.slice(i, i + 64))

console.log(`Generated ${secretsBase16.length / 64} secrets worth ${totalBigInt} wei in ${end - start}ms`)