use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingGeneric(Object<libnm_sys::NMSettingGeneric, libnm_sys::NMSettingGenericClass, NMSettingGenericClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_generic_get_type() as usize,
    }
);
