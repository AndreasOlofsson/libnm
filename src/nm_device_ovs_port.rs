use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceOvsPort(Object<libnm_sys::NMDeviceOvsPort, libnm_sys::NMDeviceOvsPortClass, NMDeviceOvsPortClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_ovs_port_get_type() as usize,
    }
);
