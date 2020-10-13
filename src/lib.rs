mod nm_access_point;
pub use nm_access_point::*;

mod nm_active_connection;
pub use nm_active_connection::*;

mod nm_checkpoint;
pub use nm_checkpoint::*;

mod nm_client;
pub use nm_client::*;

mod nm_connection;
pub use nm_connection::*;

mod nm_device_6lowpan;
pub use nm_device_6lowpan::*;

mod nm_device_adsl;
pub use nm_device_adsl::*;

mod nm_device_bond;
pub use nm_device_bond::*;

mod nm_device_bridge;
pub use nm_device_bridge::*;

mod nm_device_bt;
pub use nm_device_bt::*;

mod nm_device_dummy;
pub use nm_device_dummy::*;

mod nm_device_ethernet;
pub use nm_device_ethernet::*;

mod nm_device_generic;
pub use nm_device_generic::*;

mod nm_device_infiniband;
pub use nm_device_infiniband::*;

mod nm_device_ip_tunnel;
pub use nm_device_ip_tunnel::*;

mod nm_device_macsec;
pub use nm_device_macsec::*;

mod nm_device_macvlan;
pub use nm_device_macvlan::*;

mod nm_device_modem;
pub use nm_device_modem::*;

mod nm_device_olpc_mesh;
pub use nm_device_olpc_mesh::*;

mod nm_device_ovs_bridge;
pub use nm_device_ovs_bridge::*;

mod nm_device_ovs_interface;
pub use nm_device_ovs_interface::*;

mod nm_device_ovs_port;
pub use nm_device_ovs_port::*;

mod nm_device_ppp;
pub use nm_device_ppp::*;

mod nm_device;
pub use nm_device::*;

mod nm_device_team;
pub use nm_device_team::*;

mod nm_device_tun;
pub use nm_device_tun::*;

mod nm_device_vlan;
pub use nm_device_vlan::*;

mod nm_device_vrf;
pub use nm_device_vrf::*;

mod nm_device_vxlan;
pub use nm_device_vxlan::*;

mod nm_device_wifi_p2p;
pub use nm_device_wifi_p2p::*;

mod nm_device_wifi;
pub use nm_device_wifi::*;

mod nm_device_wimax;
pub use nm_device_wimax::*;

mod nm_device_wireguard;
pub use nm_device_wireguard::*;

mod nm_device_wpan;
pub use nm_device_wpan::*;

mod nm_dhcp_config;
pub use nm_dhcp_config::*;

mod nm_ip_address;
pub use nm_ip_address::*;

mod nm_ip_config;
pub use nm_ip_config::*;

mod nm_object;
pub use nm_object::*;

mod nm_remote_connection;
pub use nm_remote_connection::*;

mod nm_secret_agent_old;
pub use nm_secret_agent_old::*;

mod nm_setting_6lowpan;
pub use nm_setting_6lowpan::*;

mod nm_setting_802_1x;
pub use nm_setting_802_1x::*;

mod nm_setting_adsl;
pub use nm_setting_adsl::*;

mod nm_setting_bluetooth;
pub use nm_setting_bluetooth::*;

mod nm_setting_bond;
pub use nm_setting_bond::*;

mod nm_setting_bridge_port;
pub use nm_setting_bridge_port::*;

mod nm_setting_bridge;
pub use nm_setting_bridge::*;

mod nm_setting_cdma;
pub use nm_setting_cdma::*;

mod nm_setting_connection;
pub use nm_setting_connection::*;

mod nm_setting_dcb;
pub use nm_setting_dcb::*;

mod nm_setting_dummy;
pub use nm_setting_dummy::*;

mod nm_setting_ethtool;
pub use nm_setting_ethtool::*;

mod nm_setting_generic;
pub use nm_setting_generic::*;

mod nm_setting_gsm;
pub use nm_setting_gsm::*;

mod nm_setting_infiniband;
pub use nm_setting_infiniband::*;

mod nm_setting_ip4_config;
pub use nm_setting_ip4_config::*;

mod nm_setting_ip6_config;
pub use nm_setting_ip6_config::*;

mod nm_setting_ip_config;
pub use nm_setting_ip_config::*;

mod nm_setting_ip_tunnel;
pub use nm_setting_ip_tunnel::*;

mod nm_setting_macsec;
pub use nm_setting_macsec::*;

mod nm_setting_macvlan;
pub use nm_setting_macvlan::*;

mod nm_setting_match;
pub use nm_setting_match::*;

mod nm_setting_olpc_mesh;
pub use nm_setting_olpc_mesh::*;

mod nm_setting_ovs_bridge;
pub use nm_setting_ovs_bridge::*;

mod nm_setting_ovs_dpdk;
pub use nm_setting_ovs_dpdk::*;

mod nm_setting_ovs_interface;
pub use nm_setting_ovs_interface::*;

mod nm_setting_ovs_patch;
pub use nm_setting_ovs_patch::*;

mod nm_setting_ovs_port;
pub use nm_setting_ovs_port::*;

mod nm_setting_pppoe;
pub use nm_setting_pppoe::*;

mod nm_setting_ppp;
pub use nm_setting_ppp::*;

mod nm_setting_proxy;
pub use nm_setting_proxy::*;

mod nm_setting;
pub use nm_setting::*;

mod nm_setting_serial;
pub use nm_setting_serial::*;

mod nm_setting_sriov;
pub use nm_setting_sriov::*;

mod nm_setting_tc_config;
pub use nm_setting_tc_config::*;

mod nm_setting_team_port;
pub use nm_setting_team_port::*;

mod nm_setting_team;
pub use nm_setting_team::*;

mod nm_setting_tun;
pub use nm_setting_tun::*;

mod nm_setting_user;
pub use nm_setting_user::*;

mod nm_setting_vlan;
pub use nm_setting_vlan::*;

mod nm_setting_vpn;
pub use nm_setting_vpn::*;

mod nm_setting_vrf;
pub use nm_setting_vrf::*;

mod nm_setting_vxlan;
pub use nm_setting_vxlan::*;

mod nm_setting_wifi_p2p;
pub use nm_setting_wifi_p2p::*;

mod nm_setting_wimax;
pub use nm_setting_wimax::*;

mod nm_setting_wired;
pub use nm_setting_wired::*;

mod nm_setting_wireguard;
pub use nm_setting_wireguard::*;

mod nm_setting_wireless;
pub use nm_setting_wireless::*;

mod nm_setting_wireless_security;
pub use nm_setting_wireless_security::*;

mod nm_setting_wpan;
pub use nm_setting_wpan::*;

mod nm_simple_connection;
pub use nm_simple_connection::*;

mod nm_vpn_connection;
pub use nm_vpn_connection::*;

mod nm_vpn_plugin_info;
pub use nm_vpn_plugin_info::*;

mod nm_vpn_plugin_old;
pub use nm_vpn_plugin_old::*;

mod nm_vpn_service_plugin;
pub use nm_vpn_service_plugin::*;

mod nm_wifi_p2p_peer;
pub use nm_wifi_p2p_peer::*;

mod nm_wimax_nsp;
pub use nm_wimax_nsp::*;

#[cfg(test)]
mod tests {
    use super::*;
    use glib::Cast;

    #[test]
    fn dev_test() {
        let client = NMClient::new().unwrap();

        for device in client.get_devices() {
            println!("{:#?}", device);

            match device.get_applied_connection() {
                Ok(connection) => {
                    for setting in connection.get_settings() {
                        if let Ok(setting) = setting.downcast::<NMSettingWireless>() {
                            println!("{:#?}", String::from_utf8_lossy(&setting.get_ssid()));
                        }
                    }
                },
                Err(error) => print!("{:#?}", error),
            }

            if let Some(config) = device.get_ip4_config() {
                for address in config.get_adresses() {
                    println!("{:#?}", address.get_address());
                }
            }
        }
    }
}