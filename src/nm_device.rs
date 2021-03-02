use crate::*;
use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMDevice(Object<libnm_sys::NMDevice, libnm_sys::NMDeviceClass, NMDeviceClass>)
        @extends NMObject;

    match fn {
        get_type => || libnm_sys::nm_device_get_type() as usize,
    }
);

impl NMDevice {
    pub fn get_applied_connection(&self) -> Result<NMConnection, glib::error::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let stash = self.to_glib_none();
            let connection = libnm_sys::nm_device_get_applied_connection(
                stash.0,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut error,
            );

            if connection.is_null() {
                Err(glib::error::Error::from_glib_full(
                    error as *mut glib_sys::GError,
                ))
            } else {
                Ok(NMConnection::from_glib_full(connection))
            }
        }
    }

    pub fn get_active_connection(&self) -> Option<NMActiveConnection> {
        unsafe {
            let stash = self.to_glib_none();
            <Option<NMActiveConnection>>::from_glib_none(
                libnm_sys::nm_device_get_active_connection(stash.0),
            )
        }
    }

    pub fn get_ip4_config(&self) -> Option<NMIPConfig> {
        unsafe {
            let stash = self.to_glib_none();
            <Option<NMIPConfig>>::from_glib_none(libnm_sys::nm_device_get_ip4_config(stash.0))
        }
    }
}
