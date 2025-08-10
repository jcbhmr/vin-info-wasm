#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(all(target_arch = "wasm32", target_os = "wasi", target_env = "p2")))]
compile_error!("This crate is designed for wasm32-wasip2");

extern crate alloc;

pub(crate) mod bindings;
pub(crate) mod types;

use crate::bindings::exports::jcbhmr::vin_info::types::{
    Guest, GuestVin, Region as GuestRegion, Vin as GuestVinOwned, VinBorrow as GuestVinBorrow,
    VinError as GuestVinError,
};
use alloc::{
    format,
    string::{String, ToString},
};
use core::{
    cell::{Cell, RefCell},
    mem::transmute,
};
use types::VinWrapper as HostVinWrapper;
use vin_info::{Region as HostRegion, Vin as HostVin, VinError as HostVinError};

pub(crate) struct Host;
impl Guest for Host {
    type Vin = HostVinWrapper;

    fn region_as_str(self_: GuestRegion) -> String {
        HostRegion::from(self_).as_str().into()
    }
    fn region_from_wmi_region(ch: u8) -> Option<GuestRegion> {
        HostRegion::from_wmi_region(ch).map(Into::into)
    }
    fn region_to_debug_string(self_: GuestRegion) -> String {
        format!("{:?}", HostRegion::from(self_))
    }
    fn region_to_string(self_: GuestRegion) -> String {
        HostRegion::from(self_).to_string()
    }
    fn vin_error_to_debug_string(self_: GuestVinError) -> String {
        format!("{:?}", HostVinError::from(self_))
    }
    fn vin_error_to_string(self_: GuestVinError) -> String {
        HostVinError::from(self_).to_string()
    }
}
