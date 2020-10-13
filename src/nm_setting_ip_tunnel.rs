use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSettingIPTunnel(Object<libnm_sys::NMSettingIPTunnel, libnm_sys::NMSettingIPTunnelClass, NMSettingIPTunnelClass>)
        @extends NMSetting;

    match fn {
        get_type => || libnm_sys::nm_setting_ip_tunnel_get_type() as usize,
    }
);
