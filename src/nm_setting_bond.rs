use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingBond(Object<libnm_sys::NMSettingBond, libnm_sys::NMSettingBondClass, NMSettingBondClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_bond_get_type() as usize,
    }
);
