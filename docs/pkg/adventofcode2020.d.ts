/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {string}
*/
export function day02_part_one(input: string): string;
/**
* @param {string} input
* @returns {string}
*/
export function day02_part_two(input: string): string;
/**
* @param {string} input
* @returns {string}
*/
export function day01_part_one(input: string): string;
/**
* @param {string} input
* @returns {string}
*/
export function day01_part_two(input: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly day02_part_one: (a: number, b: number, c: number) => void;
  readonly day02_part_two: (a: number, b: number, c: number) => void;
  readonly day01_part_one: (a: number, b: number, c: number) => void;
  readonly day01_part_two: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        