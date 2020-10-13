use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingWirelessSecurity(Object<libnm_sys::NMSettingWirelessSecurity, libnm_sys::NMSettingWirelessSecurityClass, NMSettingWirelessSecurityClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_wireless_security_get_type() as usize,
    }
);
