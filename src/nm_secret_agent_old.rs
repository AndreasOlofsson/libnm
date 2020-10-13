use glib::translate::*;

glib::glib_wrapper!(
    pub struct NMSecretAgentOld(Object<libnm_sys::NMSecretAgentOld, libnm_sys::NMSecretAgentOldClass, NMSecretAgentOldClass>);

    match fn {
        get_type => || libnm_sys::nm_secret_agent_old_get_type() as usize,
    }
);
