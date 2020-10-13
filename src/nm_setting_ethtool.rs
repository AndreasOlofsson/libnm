use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingEthtool(Object<libnm_sys::NMSettingEthtool, libnm_sys::NMSettingEthtoolClass, NMSettingEthtoolClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ethtool_get_type() as usize,
    }
);
