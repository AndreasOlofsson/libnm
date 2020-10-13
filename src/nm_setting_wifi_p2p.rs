use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingWifiP2P(Object<libnm_sys::NMSettingWifiP2P, libnm_sys::NMSettingWifiP2PClass, NMSettingWifiP2PClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_wifi_p2p_get_type() as usize,
    }
);
