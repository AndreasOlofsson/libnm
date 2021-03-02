use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMIPConfig(Object<libnm_sys::NMIPConfig, libnm_sys::NMIPConfigClass, NMIPConfigClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_ip_config_get_type() as usize,
    }
);

impl NMIPConfig {
    pub fn get_addresses(&self) -> Vec<NMIPAddress> {
        unsafe {
            let stash = self.to_glib_none();
            NMIPAddress::from_glib_none_as_vec(libnm_sys::nm_ip_config_get_addresses(stash.0))
        }
    }
}
