/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const encrypt_descriptor: (a: number, b: number) => [number, number, number, number];
export const decrypt_descriptor: (a: number, b: number, c: number, d: number) => [number, number, number, number];
export const get_descriptor_template: (a: number, b: number) => [number, number, number, number];
export const rustsecp256k1_v0_10_0_context_create: (a: number) => number;
export const rustsecp256k1_v0_10_0_context_destroy: (a: number) => void;
export const rustsecp256k1_v0_10_0_default_illegal_callback_fn: (a: number, b: number) => void;
export const rustsecp256k1_v0_10_0_default_error_callback_fn: (a: number, b: number) => void;
export const __wbindgen_export_0: WebAssembly.Table;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
