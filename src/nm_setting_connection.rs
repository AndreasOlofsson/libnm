use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingConnection(Object<libnm_sys::NMSettingConnection, libnm_sys::NMSettingConnectionClass, NMSettingConnectionClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_connection_get_type() as usize,
    }
);
