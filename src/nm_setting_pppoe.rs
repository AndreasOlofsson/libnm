use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingPppoe(Object<libnm_sys::NMSettingPppoe, libnm_sys::NMSettingPppoeClass, NMSettingPppoeClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_pppoe_get_type() as usize,
    }
);
