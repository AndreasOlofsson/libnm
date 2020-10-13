use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceMacvlan(Object<libnm_sys::NMDeviceMacvlan, libnm_sys::NMDeviceMacvlanClass, NMDeviceMacvlanClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_macvlan_get_type() as usize,
    }
);
