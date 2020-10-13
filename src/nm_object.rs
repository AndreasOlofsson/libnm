use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMObject(Object<libnm_sys::NMObject, libnm_sys::NMObjectClass, NMObjectClass>);

    match fn {
        get_type => || libnm_sys::nm_object_get_type() as usize,
    }
);
