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
    pub fn get_adresses(&self) -> Vec<NMIPAddress> {
        unsafe {
            NMIPAddress::from_glib_none_as_vec(libnm_sys::nm_ip_config_get_addresses(self.to_glib_none().0))
        }
    }
}