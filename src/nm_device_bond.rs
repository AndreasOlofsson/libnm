use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceBond(Object<libnm_sys::NMDeviceBond, libnm_sys::NMDeviceBondClass, NMDeviceBondClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_bond_get_type() as usize,
    }
);
