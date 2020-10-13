use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingMacvlan(Object<libnm_sys::NMSettingMacvlan, libnm_sys::NMSettingMacvlanClass, NMSettingMacvlanClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_macvlan_get_type() as usize,
    }
);
