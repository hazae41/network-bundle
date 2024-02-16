export * from "../../../wasm/pkg/bundle.js";

import { __wbg_init, InitOutput } from "../../../wasm/pkg/bundle.js";
import { data } from "../../../wasm/pkg/bundle.wasm.js";

let output: InitOutput | undefined = undefined

export async function initBundledOnce() {
  return output ??= await __wbg_init(data)
}
