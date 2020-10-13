use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceWireGuard(Object<libnm_sys::NMDeviceWireGuard, libnm_sys::NMDeviceWireGuardClass, NMDeviceWireGuardClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_wireguard_get_type() as usize,
    }
);
