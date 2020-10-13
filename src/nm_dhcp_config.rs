use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDhcpConfig(Object<libnm_sys::NMDhcpConfig, libnm_sys::NMDhcpConfigClass, NMDhcpConfigClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_dhcp_config_get_type() as usize,
    }
);
