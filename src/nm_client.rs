use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMClient(Object<libnm_sys::NMClient, libnm_sys::NMClientClass, NMClientClass>);

    match fn {
        get_type => || libnm_sys::nm_client_get_type() as usize,
    }
);

impl NMClient {
    /// Creates a new NMClient.
    pub fn new() -> Result<Self, glib::error::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let client = libnm_sys::nm_client_new(std::ptr::null_mut(), &mut error);

            if client.is_null() {
                Err(glib::error::Error::from_glib_full(
                    error as *mut glib_sys::GError,
                ))
            } else {
                Ok(NMClient::from_glib_full(client))
            }
        }
    }

    /// Gets all the known network devices.
    pub fn get_devices(&self) -> Vec<NMDevice> {
        unsafe {
            let devices = libnm_sys::nm_client_get_devices(self.to_glib_none().0);

            NMDevice::from_glib_none_as_vec(devices)
        }
    }
}
