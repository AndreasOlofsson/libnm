use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceInfiniband(Object<libnm_sys::NMDeviceInfiniband, libnm_sys::NMDeviceInfinibandClass, NMDeviceInfinibandClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_infiniband_get_type() as usize,
    }
);
