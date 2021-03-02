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
            let stash = self.to_glib_none();
            let settings = libnm_sys::nm_connection_get_settings(stash.0, std::ptr::null_mut());

            NMSetting::from_glib_container_as_vec(settings)
        }
    }
}
