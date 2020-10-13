use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingIP6Config(Object<libnm_sys::NMSettingIP6Config, libnm_sys::NMSettingIP6ConfigClass, NMSettingIP6ConfigClass>)
        @extends NMSettingIPConfig, NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ip6_config_get_type() as usize,
    }
);
