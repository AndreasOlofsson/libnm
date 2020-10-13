use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMWifiP2PPeer(Object<libnm_sys::NMWifiP2PPeer, libnm_sys::NMWifiP2PPeerClass, NMWifiP2PPeerClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_wifi_p2p_peer_get_type() as usize,
    }
);
