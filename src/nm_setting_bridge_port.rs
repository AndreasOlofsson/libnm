use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingBridgePort(Object<libnm_sys::NMSettingBridgePort, libnm_sys::NMSettingBridgePortClass, NMSettingBridgePortClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_bridge_port_get_type() as usize,
    }
);
