use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingInfiniband(Object<libnm_sys::NMSettingInfiniband, libnm_sys::NMSettingInfinibandClass, NMSettingInfinibandClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_infiniband_get_type() as usize,
    }
);
