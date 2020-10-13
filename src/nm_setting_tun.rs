use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingTun(Object<libnm_sys::NMSettingTun, libnm_sys::NMSettingTunClass, NMSettingTunClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_tun_get_type() as usize,
    }
);
