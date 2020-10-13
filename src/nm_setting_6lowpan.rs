use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSetting6Lowpan(Object<libnm_sys::NMSetting6Lowpan, libnm_sys::NMSetting6LowpanClass, NMSetting6LowpanClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_6lowpan_get_type() as usize,
    }
);
