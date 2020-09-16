use crate::config::config::ScyllaConfig;
use crate::errors::errors::*;
use crate::util::dns;

use cdrs::{
    authenticators::StaticPasswordAuthenticator,
    cluster::{
        session::{new_lz4 as new_lz4_session, Session},
        ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool,
    },
    load_balancing::RoundRobin,
    query::*,
};
use fomat_macros::fomat;
use std::sync::Arc;
use tracing::info;

pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<StaticPasswordAuthenticator>>>;

fn get_nodes_ip(config: &ScyllaConfig) -> Vec<String> {
    let mut nodes_ip_port: Vec<String> = Vec::with_capacity(config.hosts.len());
    for hosts in config.hosts.iter() {
        let ip = dns::get_host_ip_from(hosts.clone());
        let ip_port = format!("{}:{}", ip, config.port);
        nodes_ip_port.push(ip_port);
    }
    nodes_ip_port
}

pub async fn create_db_session(config: &ScyllaConfig) -> std::result::Result<CurrentSession, AppError> {
    let auth = StaticPasswordAuthenticator::new(&config.user, &config.password);

    let nodes_ip_port = get_nodes_ip(config);

    let mut nodes = Vec::with_capacity(config.hosts.len());
    for node_ip_port in nodes_ip_port.iter() {
        let node_config = NodeTcpConfigBuilder::new(node_ip_port.as_str(), auth.clone())
            .build();
        nodes.push(node_config);
    }

    let cluster_config = ClusterTcpConfig(nodes);
    let session = new_lz4_session(&cluster_config, RoundRobin::new());
    Ok(session.await?)
}

pub async fn create_keyspace(
    session: Arc<CurrentSession>,
    keyspace: &str,
    replication_factor: i8,
) -> std::result::Result<(), AppError> {
    let create_keyspace = fomat!(
        ("CREATE KEYSPACE IF NOT EXISTS ")(keyspace)("\n")
        "\t WITH REPLICATION = { \n"
        "\t 'class': 'SimpleStrategy', \n"
        ("\t 'replication_factor': ")(replication_factor)("\n")
        "};"
    );
    info!("creating keyspace: {}", create_keyspace);
    session.query(create_keyspace).await?;
    Ok(())
}
