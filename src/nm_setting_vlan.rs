use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingVlan(Object<libnm_sys::NMSettingVlan, libnm_sys::NMSettingVlanClass, NMSettingVlanClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_vlan_get_type() as usize,
    }
);
