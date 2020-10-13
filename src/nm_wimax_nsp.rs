use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMWimaxNsp(Object<libnm_sys::NMWimaxNsp, libnm_sys::NMWimaxNspClass, NMWimaxNspClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_wimax_nsp_get_type() as usize,
    }
);
