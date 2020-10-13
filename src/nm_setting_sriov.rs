use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingSriov(Object<libnm_sys::NMSettingSriov, libnm_sys::NMSettingSriovClass, NMSettingSriovClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_sriov_get_type() as usize,
    }
);
