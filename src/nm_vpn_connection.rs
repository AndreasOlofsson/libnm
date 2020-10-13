use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMVpnConnection(Object<libnm_sys::NMVpnConnection, libnm_sys::NMVpnConnectionClass, NMVpnConnectionClass>)
        @extends NMActiveConnection, NMObject;

    match fn {
        get_type => || libnm_sys::nm_vpn_connection_get_type() as usize,
    }
);
