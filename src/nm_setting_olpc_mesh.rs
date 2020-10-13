use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingOlpcMesh(Object<libnm_sys::NMSettingOlpcMesh, libnm_sys::NMSettingOlpcMeshClass, NMSettingOlpcMeshClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_olpc_mesh_get_type() as usize,
    }
);
