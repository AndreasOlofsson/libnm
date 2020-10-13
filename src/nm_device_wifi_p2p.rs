use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceWifiP2P(Object<libnm_sys::NMDeviceWifiP2P, libnm_sys::NMDeviceWifiP2PClass, NMDeviceWifiP2PClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_wifi_p2p_get_type() as usize,
    }
);
