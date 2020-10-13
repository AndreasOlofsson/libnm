use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingBluetooth(Object<libnm_sys::NMSettingBluetooth, libnm_sys::NMSettingBluetoothClass, NMSettingBluetoothClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_bluetooth_get_type() as usize,
    }
);
