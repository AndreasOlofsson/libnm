use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMConnection(Interface<libnm_sys::NMConnection>);

    match fn {
        get_type => || libnm_sys::nm_connection_get_type() as usize,
    }
);

impl NMConnection {
    pub fn get_settings(&self) -> Vec<NMSetting> {
        unsafe {
            let settings = libnm_sys::nm_connection_get_settings(self.to_glib_none().0, std::ptr::null_mut());

            NMSetting::from_glib_none_as_vec(settings)
        }
    }
}