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
use vin_info::{Region as HostRegion, Vin as HostVin, VinError as HostVinError};

impl From<HostRegion> for GuestRegion {
    fn from(value: HostRegion) -> Self {
        match value {
            HostRegion::Africa => Self::Africa,
            HostRegion::Asia => Self::Asia,
            HostRegion::Europe => Self::Europe,
            HostRegion::NorthAmerica => Self::NorthAmerica,
            HostRegion::Oceania => Self::Oceania,
            HostRegion::SouthAmerica => Self::SouthAmerica,
        }
    }
}
impl From<GuestRegion> for HostRegion {
    fn from(value: GuestRegion) -> Self {
        match value {
            GuestRegion::Africa => Self::Africa,
            GuestRegion::Asia => Self::Asia,
            GuestRegion::Europe => Self::Europe,
            GuestRegion::NorthAmerica => Self::NorthAmerica,
            GuestRegion::Oceania => Self::Oceania,
            GuestRegion::SouthAmerica => Self::SouthAmerica,
        }
    }
}

impl From<HostVinError> for GuestVinError {
    fn from(value: HostVinError) -> Self {
        match value {
            HostVinError::InvalidLen => Self::InvalidLen,
            HostVinError::InvalidChar(idx, ch) => Self::InvalidChar((idx as u32, ch as u32)),
        }
    }
}
impl From<GuestVinError> for HostVinError {
    fn from(value: GuestVinError) -> Self {
        match value {
            GuestVinError::InvalidLen => Self::InvalidLen,
            GuestVinError::InvalidChar((idx, ch)) => {
                Self::InvalidChar(idx as usize, char::from_u32(ch).unwrap())
            }
        }
    }
}

unsafe fn transmute_static<T: ?Sized>(value: &T) -> &'static T {
    unsafe { transmute(value) }
}

pub(crate) struct VinWrapper(RefCell<(HostVin<'static>, String)>);
impl From<HostVin<'_>> for VinWrapper {
    fn from(value: HostVin<'_>) -> Self {
        let storage = value.to_string();
        let storage_str = unsafe { transmute_static(storage.as_str()) };
        let inner = HostVin::new(storage_str);
        Self(RefCell::new((inner, storage)))
    }
}

impl GuestVin for VinWrapper {
    fn calculate_checksum(&self) -> u32 {
        self.0.borrow().0.calculate_checksum()
    }
    fn calculate_checksum_digit(&self) -> u32 {
        self.0.borrow().0.calculate_checksum_digit() as u32
    }
    fn is_checksum_valid(&self) -> bool {
        self.0.borrow().0.is_checksum_valid()
    }
    fn manufacturer_country(&self) -> String {
        self.0.borrow().0.manufacturer_country().into()
    }
    fn manufacturer_name(&self) -> String {
        self.0.borrow().0.manufacturer_name().into()
    }
    fn manufacturer_region(&self) -> Option<GuestRegion> {
        self.0.borrow().0.manufacturer_region().map(Into::into)
    }
    fn new(vin: String) -> Self {
        let storage_str = unsafe { transmute_static(vin.as_str()) };
        let inner = HostVin::new(storage_str);
        Self(RefCell::new((inner, vin)))
    }
    fn try_new(vin: String) -> Result<GuestVinOwned, GuestVinError> {
        let storage_str = unsafe { transmute_static(vin.as_str()) };
        HostVin::try_new(storage_str)
            .map(|inner| {
                let wrapped = Self(RefCell::new((inner, vin)));
                GuestVinOwned::new(wrapped)
            })
            .map_err(Into::into)
    }
    fn vds(&self) -> String {
        self.0.borrow().0.vds().into()
    }
    fn vic(&self) -> String {
        self.0.borrow().0.vic().into()
    }
    fn wmi(&self) -> String {
        self.0.borrow().0.wmi().into()
    }

    fn clone(&self) -> GuestVinOwned {
        let storage_clone = self.0.borrow().1.clone();
        let storage_str = unsafe { transmute_static(storage_clone.as_str()) };
        let inner_clone = HostVin::new(storage_str);
        let wrapped_clone = Self(RefCell::new((inner_clone, storage_clone)));
        GuestVinOwned::new(wrapped_clone)
    }
    fn clone_from(&self, vin: GuestVinBorrow<'_>) -> () {
        self.0
            .borrow_mut()
            .1
            .clone_from(&vin.get::<Self>().0.borrow().1);
        let storage_ref = &vin.get::<Self>().0.borrow().1;
        let storage_str = unsafe { transmute_static(storage_ref.as_str()) };
        self.0.borrow_mut().0 = HostVin::new(storage_str);
    }

    fn to_debug_string(&self) -> String {
        format!("{:?}", self.0.borrow().0)
    }

    fn to_string(&self) -> String {
        self.0.borrow().0.to_string()
    }

    fn eq(&self, other: GuestVinBorrow<'_>) -> bool {
        self.0.borrow().0 == other.get::<Self>().0.borrow().0
    }
    fn ne(&self, other: GuestVinBorrow<'_>) -> bool {
        self.0.borrow().0 != other.get::<Self>().0.borrow().0
    }
}
