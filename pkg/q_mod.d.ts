/* tslint:disable */
/* eslint-disable */
/**
* @param {bigint} u
* @param {bigint} m
* @param {boolean} debug
* @returns {Quotient}
*/
export function qmod_wasm(u: bigint, m: bigint, debug: boolean): Quotient;
/**
*/
export class Quotient {
  free(): void;
/**
* @returns {bigint}
*/
  get_num(): bigint;
/**
* @returns {bigint}
*/
  get_div(): bigint;
/**
*/
  0: bigint;
/**
*/
  1: bigint;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_quotient_free: (a: number) => void;
  readonly __wbg_get_quotient_0: (a: number) => number;
  readonly __wbg_set_quotient_0: (a: number, b: number) => void;
  readonly __wbg_get_quotient_1: (a: number) => number;
  readonly __wbg_set_quotient_1: (a: number, b: number) => void;
  readonly qmod_wasm: (a: number, b: number, c: number) => number;
  readonly quotient_get_num: (a: number) => number;
  readonly quotient_get_div: (a: number) => number;
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
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
