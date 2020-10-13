use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingBridge(Object<libnm_sys::NMSettingBridge, libnm_sys::NMSettingBridgeClass, NMSettingBridgeClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_bridge_get_type() as usize,
    }
);
