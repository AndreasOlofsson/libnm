use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMActiveConnection(Object<libnm_sys::NMActiveConnection, libnm_sys::NMActiveConnectionClass, NMActiveConnectionClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_active_connection_get_type() as usize,
    }
);
