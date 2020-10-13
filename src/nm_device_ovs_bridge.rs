use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceOvsBridge(Object<libnm_sys::NMDeviceOvsBridge, libnm_sys::NMDeviceOvsBridgeClass, NMDeviceOvsBridgeClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_ovs_bridge_get_type() as usize,
    }
);
