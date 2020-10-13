use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceMacsec(Object<libnm_sys::NMDeviceMacsec, libnm_sys::NMDeviceMacsecClass, NMDeviceMacsecClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_macsec_get_type() as usize,
    }
);
