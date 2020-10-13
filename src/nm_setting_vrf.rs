use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingVrf(Object<libnm_sys::NMSettingVrf, libnm_sys::NMSettingVrfClass, NMSettingVrfClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_vrf_get_type() as usize,
    }
);
