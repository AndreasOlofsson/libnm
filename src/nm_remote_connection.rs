use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMRemoteConnection(Object<libnm_sys::NMRemoteConnection, libnm_sys::NMRemoteConnectionClass, NMRemoteConnectionClass>)
        @extends NMObject,
        @implements NMConnection;

    match fn {
        get_type => || libnm_sys::nm_remote_connection_get_type() as usize,
    }
);
