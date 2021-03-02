use glib::glib_boxed_wrapper;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMIPAddress(Boxed<libnm_sys::NMIPAddress>);

    match fn {
        // ref => |ptr| {
        //     libnm_sys::nm_ip_address_ref(ptr as *mut _);
        //     ptr as *mut _
        // },
        // unref => |ptr| libnm_sys::nm_ip_address_unref(ptr),
        copy => |ptr| { libnm_sys::nm_ip_address_ref(ptr as *mut _); ptr as *mut _ },
        free => |ptr| libnm_sys::nm_ip_address_unref(ptr as *mut _),
        get_type => || libnm_sys::nm_ip_address_get_type() as usize,
    }
);

impl NMIPAddress {
    pub fn get_address(&self) -> String {
        unsafe {
            let stash = self.to_glib_none();
            String::from_glib_none(libnm_sys::nm_ip_address_get_address(stash.0 as *mut _))
        }
    }
}
