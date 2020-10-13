use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingVpn(Object<libnm_sys::NMSettingVpn, libnm_sys::NMSettingVpnClass, NMSettingVpnClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_vpn_get_type() as usize,
    }
);
