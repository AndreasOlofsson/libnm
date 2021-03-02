use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMActiveConnection(Object<libnm_sys::NMActiveConnection, libnm_sys::NMActiveConnectionClass, NMActiveConnectionClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_active_connection_get_type() as usize,
    }
);

impl NMActiveConnection {
    pub fn get_connection(&self) -> NMRemoteConnection {
        unsafe {
            let stash = self.to_glib_none();
            NMRemoteConnection::from_glib_none(libnm_sys::nm_active_connection_get_connection(
                stash.0,
            ))
        }
    }
}
