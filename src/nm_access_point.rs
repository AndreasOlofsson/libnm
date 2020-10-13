use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMAccessPoint(Object<libnm_sys::NMAccessPoint, libnm_sys::NMAccessPointClass, NMAccessPointClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_access_point_get_type() as usize,
    }
);
