use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDevice6Lowpan(Object<libnm_sys::NMDevice6Lowpan, libnm_sys::NMDevice6LowpanClass, NMDevice6LowpanClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_6lowpan_get_type() as usize,
    }
);
