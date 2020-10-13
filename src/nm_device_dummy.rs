use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceDummy(Object<libnm_sys::NMDeviceDummy, libnm_sys::NMDeviceDummyClass, NMDeviceDummyClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_dummy_get_type() as usize,
    }
);
