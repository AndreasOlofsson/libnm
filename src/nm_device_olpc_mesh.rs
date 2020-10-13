use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceOlpcMesh(Object<libnm_sys::NMDeviceOlpcMesh, libnm_sys::NMDeviceOlpcMeshClass, NMDeviceOlpcMeshClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_olpc_mesh_get_type() as usize,
    }
);
