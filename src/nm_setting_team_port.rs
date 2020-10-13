use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingTeamPort(Object<libnm_sys::NMSettingTeamPort, libnm_sys::NMSettingTeamPortClass, NMSettingTeamPortClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_team_port_get_type() as usize,
    }
);
