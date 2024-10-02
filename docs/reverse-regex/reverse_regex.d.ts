/* tslint:disable */
/* eslint-disable */
/**
* @param {string} pattern
* @param {EngineConfig | undefined} [configs]
* @returns {string}
*/
export function generate(pattern: string, configs?: EngineConfig): string;
/**
* @param {string} pattern
* @returns {string}
*/
export function hello(pattern: string): string;
/**
*/
export class EngineConfig {
  free(): void;
/**
* @param {boolean} force_decimal
* @param {boolean} force_alphanumeric
* @param {number} max_iterations
*/
  constructor(force_decimal: boolean, force_alphanumeric: boolean, max_iterations: number);
/**
*/
  force_alphanumeric: boolean;
/**
*/
  force_decimal: boolean;
/**
*/
  max_iterations: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly generate: (a: number, b: number, c: number, d: number) => void;
  readonly hello: (a: number, b: number, c: number) => void;
  readonly __wbg_engineconfig_free: (a: number, b: number) => void;
  readonly __wbg_get_engineconfig_force_decimal: (a: number) => number;
  readonly __wbg_set_engineconfig_force_decimal: (a: number, b: number) => void;
  readonly __wbg_get_engineconfig_force_alphanumeric: (a: number) => number;
  readonly __wbg_set_engineconfig_force_alphanumeric: (a: number, b: number) => void;
  readonly __wbg_get_engineconfig_max_iterations: (a: number) => number;
  readonly __wbg_set_engineconfig_max_iterations: (a: number, b: number) => void;
  readonly engineconfig_new: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
