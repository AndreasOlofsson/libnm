use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDevicePpp(Object<libnm_sys::NMDevicePpp, libnm_sys::NMDevicePppClass, NMDevicePppClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_ppp_get_type() as usize,
    }
);
