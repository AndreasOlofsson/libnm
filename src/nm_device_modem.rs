use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceModem(Object<libnm_sys::NMDeviceModem, libnm_sys::NMDeviceModemClass, NMDeviceModemClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_modem_get_type() as usize,
    }
);
