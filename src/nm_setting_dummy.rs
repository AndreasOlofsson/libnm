use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingDummy(Object<libnm_sys::NMSettingDummy, libnm_sys::NMSettingDummyClass, NMSettingDummyClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_dummy_get_type() as usize,
    }
);
