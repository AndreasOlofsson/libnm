use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingPpp(Object<libnm_sys::NMSettingPpp, libnm_sys::NMSettingPppClass, NMSettingPppClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ppp_get_type() as usize,
    }
);
