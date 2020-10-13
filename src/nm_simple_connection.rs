use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSimpleConnection(Object<libnm_sys::NMSimpleConnection, libnm_sys::NMSimpleConnectionClass, NMSimpleConnectionClass>);

    match fn {
        get_type => || libnm_sys::nm_simple_connection_get_type() as usize,
    }
);
