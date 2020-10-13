use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMVpnServicePlugin(Object<libnm_sys::NMVpnServicePlugin, libnm_sys::NMVpnServicePluginClass, NMVpnServicePluginClass>);

    match fn {
        get_type => || libnm_sys::nm_vpn_service_plugin_get_type() as usize,
    }
);
