use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingOvsBridge(Object<libnm_sys::NMSettingOvsBridge, libnm_sys::NMSettingOvsBridgeClass, NMSettingOvsBridgeClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ovs_bridge_get_type() as usize,
    }
);
