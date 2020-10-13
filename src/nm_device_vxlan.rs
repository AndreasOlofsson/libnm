use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceVxlan(Object<libnm_sys::NMDeviceVxlan, libnm_sys::NMDeviceVxlanClass, NMDeviceVxlanClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_vxlan_get_type() as usize,
    }
);
