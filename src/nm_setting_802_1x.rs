use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSetting8021x(Object<libnm_sys::NMSetting8021x, libnm_sys::NMSetting8021xClass, NMSetting8021xClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_802_1x_get_type() as usize,
    }
);
