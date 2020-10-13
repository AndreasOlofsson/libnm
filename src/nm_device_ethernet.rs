use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceEthernet(Object<libnm_sys::NMDeviceEthernet, libnm_sys::NMDeviceEthernetClass, NMDeviceEthernetClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_ethernet_get_type() as usize,
    }
);
