use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceTun(Object<libnm_sys::NMDeviceTun, libnm_sys::NMDeviceTunClass, NMDeviceTunClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_tun_get_type() as usize,
    }
);
