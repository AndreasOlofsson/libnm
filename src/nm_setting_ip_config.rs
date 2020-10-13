use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingIPConfig(Object<libnm_sys::NMSettingIPConfig, libnm_sys::NMSettingIPConfigClass, NMSettingIPConfigClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ip_config_get_type() as usize,
    }
);
