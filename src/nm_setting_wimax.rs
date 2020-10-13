use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingWimax(Object<libnm_sys::NMSettingWimax, libnm_sys::NMSettingWimaxClass, NMSettingWimaxClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_wimax_get_type() as usize,
    }
);
