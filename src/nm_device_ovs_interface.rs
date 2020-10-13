use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceOvsInterface(Object<libnm_sys::NMDeviceOvsInterface, libnm_sys::NMDeviceOvsInterfaceClass, NMDeviceOvsInterfaceClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_ovs_interface_get_type() as usize,
    }
);
