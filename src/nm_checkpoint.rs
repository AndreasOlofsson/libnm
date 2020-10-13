use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMCheckpoint(Object<libnm_sys::NMCheckpoint, libnm_sys::NMCheckpointClass, NMCheckpointClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_checkpoint_get_type() as usize,
    }
);
