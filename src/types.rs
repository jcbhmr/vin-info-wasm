use vin_info::{Region as HostRegion, Vin as HostVin, VinError as HostVinError};
use alloc::{borrow::ToOwned, string::String};
use crate::bindings::exports::jcbhmr::vin_info::types::{Guest, GuestVin, Region, Vin, VinBorrow, VinError};

pub(crate) struct HostVinWrapper {
    storage: String,
    inner: HostVin<'static>,
}
impl From<HostVin<'_>> for HostVinWrapper {
    fn from(value: HostVin<'_>) -> Self {
        let storage = value.to_string();
        let inner = HostVin::new(storage.as_str());
        Self { storage, inner }
    }
}

impl GuestVin for HostVinWrapper {
    fn calculate_checksum(&self,) -> u32 {
        
    }
    fn calculate_checksum_digit(&self,) -> u32 {
        
    }
    
    fn clone(&self,) -> Vin {
        
    }
    fn clone_from(&self,vin:VinBorrow<'_>,) -> () {
        
    }
}

pub(crate) struct Host;
impl Guest for Host {
    type Vin = HostVinWrapper;

    fn region_as_str(self_:Region,) -> String {
        
    }
    fn region_from_wmi_region(ch:u8,) -> Option<Region> {
        
    }
    fn region_to_debug_string(self_:Region,) -> String {
        
    }
    fn region_to_string(self_:Region,) -> String {
        
    }
    fn vin_error_to_debug_string(self_:VinError,) -> String {
        
    }
    fn vin_error_to_string(self_:VinError,) -> String {
        
    }
}