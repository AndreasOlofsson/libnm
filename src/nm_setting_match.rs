use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingMatch(Object<libnm_sys::NMSettingMatch, libnm_sys::NMSettingMatchClass, NMSettingMatchClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_match_get_type() as usize,
    }
);
