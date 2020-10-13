use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingAdsl(Object<libnm_sys::NMSettingAdsl, libnm_sys::NMSettingAdslClass, NMSettingAdslClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_adsl_get_type() as usize,
    }
);
