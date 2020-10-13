use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingTCConfig(Object<libnm_sys::NMSettingTCConfig, libnm_sys::NMSettingTCConfigClass, NMSettingTCConfigClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_tc_config_get_type() as usize,
    }
);
