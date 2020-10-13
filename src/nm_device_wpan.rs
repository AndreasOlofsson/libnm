use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceWpan(Object<libnm_sys::NMDeviceWpan, libnm_sys::NMDeviceWpanClass, NMDeviceWpanClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_wpan_get_type() as usize,
    }
);
