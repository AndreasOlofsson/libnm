use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingMacsec(Object<libnm_sys::NMSettingMacsec, libnm_sys::NMSettingMacsecClass, NMSettingMacsecClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_macsec_get_type() as usize,
    }
);
