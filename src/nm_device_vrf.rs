use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceVrf(Object<libnm_sys::NMDeviceVrf, libnm_sys::NMDeviceVrfClass, NMDeviceVrfClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_vrf_get_type() as usize,
    }
);
