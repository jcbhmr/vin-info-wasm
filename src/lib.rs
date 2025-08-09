#![cfg(all(target_arch = "wasm32", target_os = "wasi", target_env = "p2"))]
#![no_std]

extern crate alloc;

pub(crate) mod bindings;
pub(crate) mod types;
