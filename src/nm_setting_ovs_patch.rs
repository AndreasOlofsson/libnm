use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingOvsPatch(Object<libnm_sys::NMSettingOvsPatch, libnm_sys::NMSettingOvsPatchClass, NMSettingOvsPatchClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ovs_patch_get_type() as usize,
    }
);
