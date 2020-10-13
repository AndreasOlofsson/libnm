use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingSerial(Object<libnm_sys::NMSettingSerial, libnm_sys::NMSettingSerialClass, NMSettingSerialClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_serial_get_type() as usize,
    }
);
