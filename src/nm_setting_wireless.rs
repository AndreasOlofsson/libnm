use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingWireless(Object<libnm_sys::NMSettingWireless, libnm_sys::NMSettingWirelessClass, NMSettingWirelessClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_wireless_get_type() as usize,
    }
);

impl NMSettingWireless {
    pub fn get_ssid(&self) -> Vec<u8> {
        unsafe {
            let stash = self.to_glib_none();
            glib::Bytes::from_glib_none(libnm_sys::nm_setting_wireless_get_ssid(stash.0)).to_vec()
        }
    }
}
