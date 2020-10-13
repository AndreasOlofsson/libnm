use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceWifi(Object<libnm_sys::NMDeviceWifi, libnm_sys::NMDeviceWifiClass, NMDeviceWifiClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_wifi_get_type() as usize,
    }
);
