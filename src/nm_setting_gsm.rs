use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingGsm(Object<libnm_sys::NMSettingGsm, libnm_sys::NMSettingGsmClass, NMSettingGsmClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_gsm_get_type() as usize,
    }
);
