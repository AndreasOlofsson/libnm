use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceWimax(Object<libnm_sys::NMDeviceWimax, libnm_sys::NMDeviceWimaxClass, NMDeviceWimaxClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_wimax_get_type() as usize,
    }
);
