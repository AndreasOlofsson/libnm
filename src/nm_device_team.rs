use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDeviceTeam(Object<libnm_sys::NMDeviceTeam, libnm_sys::NMDeviceTeamClass, NMDeviceTeamClass>)
        @extends NMDevice, NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_team_get_type() as usize,
    }
);
