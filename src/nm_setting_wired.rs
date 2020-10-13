use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingWired(Object<libnm_sys::NMSettingWired, libnm_sys::NMSettingWiredClass, NMSettingWiredClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_wired_get_type() as usize,
    }
);
