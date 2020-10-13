use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingOvsDpdk(Object<libnm_sys::NMSettingOvsDpdk, libnm_sys::NMSettingOvsDpdkClass, NMSettingOvsDpdkClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ovs_dpdk_get_type() as usize,
    }
);
