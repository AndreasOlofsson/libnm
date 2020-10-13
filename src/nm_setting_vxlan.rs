use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingVxlan(Object<libnm_sys::NMSettingVxlan, libnm_sys::NMSettingVxlanClass, NMSettingVxlanClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_vxlan_get_type() as usize,
    }
);
