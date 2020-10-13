use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceBt(Object<libnm_sys::NMDeviceBt, libnm_sys::NMDeviceBtClass, NMDeviceBtClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_bt_get_type() as usize,
    }
);
