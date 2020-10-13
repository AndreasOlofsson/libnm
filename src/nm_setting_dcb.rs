use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingDcb(Object<libnm_sys::NMSettingDcb, libnm_sys::NMSettingDcbClass, NMSettingDcbClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_dcb_get_type() as usize,
    }
);
