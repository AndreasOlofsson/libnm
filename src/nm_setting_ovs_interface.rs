use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingOvsInterface(Object<libnm_sys::NMSettingOvsInterface, libnm_sys::NMSettingOvsInterfaceClass, NMSettingOvsInterfaceClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ovs_interface_get_type() as usize,
    }
);
