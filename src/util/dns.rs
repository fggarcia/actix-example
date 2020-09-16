use dns_lookup::lookup_host;
use tracing::info;

pub fn get_host_ip_from(hostname: String) -> String {
    info!("ip: {}", hostname);
    let first_node = lookup_host(hostname.as_str()).unwrap()[0];
    let first_node_hostname = first_node.to_string();
    info!("first node hostname: {}", first_node_hostname);
    first_node_hostname
}