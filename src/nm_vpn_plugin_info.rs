use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMVpnPluginInfo(Object<libnm_sys::NMVpnPluginInfo, libnm_sys::NMVpnPluginInfoClass, NMVpnPluginInfoClass>);

    match fn {
        get_type => || libnm_sys::nm_vpn_plugin_info_get_type() as usize,
    }
);
