use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingUser(Object<libnm_sys::NMSettingUser, libnm_sys::NMSettingUserClass, NMSettingUserClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_user_get_type() as usize,
    }
);
