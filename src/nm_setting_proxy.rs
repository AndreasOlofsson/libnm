use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingProxy(Object<libnm_sys::NMSettingProxy, libnm_sys::NMSettingProxyClass, NMSettingProxyClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_proxy_get_type() as usize,
    }
);
