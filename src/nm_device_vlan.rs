use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceVlan(Object<libnm_sys::NMDeviceVlan, libnm_sys::NMDeviceVlanClass, NMDeviceVlanClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_vlan_get_type() as usize,
    }
);
