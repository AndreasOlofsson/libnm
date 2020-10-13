use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingIP4Config(Object<libnm_sys::NMSettingIP4Config, libnm_sys::NMSettingIP4ConfigClass, NMSettingIP4ConfigClass>)
        @extends NMSettingIPConfig, NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ip4_config_get_type() as usize,
    }
);
