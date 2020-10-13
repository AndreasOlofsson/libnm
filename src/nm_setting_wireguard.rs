use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingWireGuard(Object<libnm_sys::NMSettingWireGuard, libnm_sys::NMSettingWireGuardClass, NMSettingWireGuardClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_wireguard_get_type() as usize,
    }
);
