use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingTeam(Object<libnm_sys::NMSettingTeam, libnm_sys::NMSettingTeamClass, NMSettingTeamClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_team_get_type() as usize,
    }
);
