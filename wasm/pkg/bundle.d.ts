/* tslint:disable */
/* eslint-disable */
/**
* @param {Memory} bytes
* @returns {string}
*/
export function base16_encode_lower(bytes: Memory): string;
/**
* @param {Memory} bytes
* @returns {string}
*/
export function base16_encode_upper(bytes: Memory): string;
/**
* @param {string} text
* @returns {Memory}
*/
export function base16_decode_mixed(text: string): Memory;
/**
* @param {string} text
* @returns {Memory}
*/
export function base16_decode_lower(text: string): Memory;
/**
* @param {string} text
* @returns {Memory}
*/
export function base16_decode_upper(text: string): Memory;
/**
* @param {Memory} data
* @returns {Memory}
*/
export function keccak256(data: Memory): Memory;
/**
*/
export class Keccak256Hasher {
  [Symbol.dispose](): void
  free(): void;
/**
*/
  constructor();
/**
* @returns {Keccak256Hasher}
*/
  clone(): Keccak256Hasher;
/**
* @param {Memory} data
*/
  update(data: Memory): void;
/**
* @returns {Memory}
*/
  finalize(): Memory;
}
/**
*/
export class Memory {
  [Symbol.dispose](): void
  free(): void;
/**
* @param {Uint8Array} inner
*/
  constructor(inner: Uint8Array);
/**
* @returns {number}
*/
  ptr(): number;
/**
* @returns {number}
*/
  len(): number;

  /**
   * Free on next tick
   **/
  freeNextTick(): Memory

  /**
   * Get the bytes in memory
   **/
  get bytes(): Uint8Array

  /**
   * Copy the bytes and free them
   **/
  copyAndDispose(): Uint8Array
}
/**
*/
export class NetworkMixin {
  [Symbol.dispose](): void
  free(): void;
/**
* @param {Memory} chain_memory
* @param {Memory} contract_memory
* @param {Memory} receiver_nonce
* @param {Memory} nonce_memory
*/
  constructor(chain_memory: Memory, contract_memory: Memory, receiver_nonce: Memory, nonce_memory: Memory);
/**
* @param {Memory} minimum_memory
* @returns {NetworkSecret}
*/
  generate(minimum_memory: Memory): NetworkSecret;
/**
* @param {Memory} secret_memory
* @returns {Memory}
*/
  verify_secret(secret_memory: Memory): Memory;
/**
* @param {Memory} proof_memory
* @returns {Memory}
*/
  verify_proof(proof_memory: Memory): Memory;
}
/**
*/
export class NetworkSecret {
  [Symbol.dispose](): void
  free(): void;
/**
* @returns {Memory}
*/
  to_secret(): Memory;
/**
* @returns {Memory}
*/
  to_proof(): Memory;
/**
* @returns {Memory}
*/
  to_value(): Memory;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_networksecret_free: (a: number) => void;
  readonly networksecret_to_secret: (a: number) => number;
  readonly networksecret_to_proof: (a: number) => number;
  readonly networksecret_to_value: (a: number) => number;
  readonly networkmixin_new: (a: number, b: number, c: number, d: number) => number;
  readonly networkmixin_generate: (a: number, b: number) => number;
  readonly networkmixin_verify_secret: (a: number, b: number) => number;
  readonly networkmixin_verify_proof: (a: number, b: number) => number;
  readonly base16_encode_lower: (a: number, b: number) => void;
  readonly base16_encode_upper: (a: number, b: number) => void;
  readonly base16_decode_mixed: (a: number, b: number, c: number) => void;
  readonly base16_decode_lower: (a: number, b: number, c: number) => void;
  readonly base16_decode_upper: (a: number, b: number, c: number) => void;
  readonly keccak256: (a: number) => number;
  readonly __wbg_keccak256hasher_free: (a: number) => void;
  readonly keccak256hasher_new: () => number;
  readonly keccak256hasher_clone: (a: number) => number;
  readonly keccak256hasher_update: (a: number, b: number) => void;
  readonly keccak256hasher_finalize: (a: number) => number;
  readonly __wbg_memory_free: (a: number) => void;
  readonly memory_new: (a: number, b: number) => number;
  readonly memory_ptr: (a: number) => number;
  readonly memory_len: (a: number) => number;
  readonly __wbg_networkmixin_free: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
