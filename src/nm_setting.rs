use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSetting(Object<libnm_sys::NMSetting, libnm_sys::NMSettingClass, NMSettingClass>);

    match fn {
        get_type => || libnm_sys::nm_setting_get_type() as usize,
    }
);

impl NMSetting {
    pub fn get_name(&self) -> String {
        unsafe {
            String::from_glib_none(libnm_sys::nm_setting_get_name(self.to_glib_none().0))
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            String::from_glib_full(libnm_sys::nm_setting_to_string(self.to_glib_none().0))
        }
    }
}
