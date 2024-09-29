/* tslint:disable */
/* eslint-disable */
/**
* @param {any} json_block_js_value
* @returns {string}
*/
export function block_hash(json_block_js_value: any): string;
/**
* @param {string} merkle_proofs_hex_str
* @param {(string)[]} path
* @returns {any}
*/
export function process_query_proofs(merkle_proofs_hex_str: string, path: (string)[]): any;
/**
*/
export class BlockValidator {
  free(): void;
/**
* @param {bigint} era_number
* @param {any} validator_weights_js_value
*/
  constructor(era_number: bigint, validator_weights_js_value: any);
/**
* @param {any} json_block_js_value
*/
  validate(json_block_js_value: any): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_blockvalidator_free: (a: number) => void;
  readonly blockvalidator_new: (a: number, b: number, c: number) => void;
  readonly blockvalidator_validate: (a: number, b: number, c: number) => void;
  readonly block_hash: (a: number, b: number) => void;
  readonly process_query_proofs: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
