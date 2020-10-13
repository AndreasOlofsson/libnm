use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingOvsPort(Object<libnm_sys::NMSettingOvsPort, libnm_sys::NMSettingOvsPortClass, NMSettingOvsPortClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ovs_port_get_type() as usize,
    }
);
