use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMVpnPluginOld(Object<libnm_sys::NMVpnPluginOld, libnm_sys::NMVpnPluginOldClass, NMVpnPluginOldClass>);

    match fn {
        get_type => || libnm_sys::nm_vpn_plugin_old_get_type() as usize,
    }
);
