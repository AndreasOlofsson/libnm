use glib::glib_shared_wrapper;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMIPAddress(Shared<libnm_sys::NMIPAddress>);

    match fn {
        ref => |ptr| {
            libnm_sys::nm_ip_address_ref(ptr as *mut _);
            ptr as *mut _
        },
        unref => |ptr| libnm_sys::nm_ip_address_unref(ptr),
        get_type => || libnm_sys::nm_ip_address_get_type() as usize,
    }
);

impl NMIPAddress {
    pub fn get_address(&self) -> String {
        unsafe {
            String::from_glib_full(libnm_sys::nm_ip_address_get_address(self.to_glib_none().0))
        }
    }
}