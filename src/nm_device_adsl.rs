use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceAdsl(Object<libnm_sys::NMDeviceAdsl, libnm_sys::NMDeviceAdslClass, NMDeviceAdslClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_adsl_get_type() as usize,
    }
);
