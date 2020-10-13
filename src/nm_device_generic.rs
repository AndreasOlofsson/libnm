use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceGeneric(Object<libnm_sys::NMDeviceGeneric, libnm_sys::NMDeviceGenericClass, NMDeviceGenericClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_generic_get_type() as usize,
    }
);
