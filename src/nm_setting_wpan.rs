use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingWpan(Object<libnm_sys::NMSettingWpan, libnm_sys::NMSettingWpanClass, NMSettingWpanClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_wpan_get_type() as usize,
    }
);
