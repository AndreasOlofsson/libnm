use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceBridge(Object<libnm_sys::NMDeviceBridge, libnm_sys::NMDeviceBridgeClass, NMDeviceBridgeClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_bridge_get_type() as usize,
    }
);
