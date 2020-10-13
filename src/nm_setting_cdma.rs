use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingCdma(Object<libnm_sys::NMSettingCdma, libnm_sys::NMSettingCdmaClass, NMSettingCdmaClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_cdma_get_type() as usize,
    }
);
