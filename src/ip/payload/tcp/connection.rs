use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::ip::addr::Addr;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SockAddr {
    pub addr: Addr,
    pub port: u16,
}

impl SockAddr {
    pub fn new(addr: Addr, port: u16) -> Self {
        SockAddr { addr, port }
    }
}

#[derive(Debug, Clone)]
pub enum TcpState {
    Listen,
    // SynSent,
    SynReceived,
    Established,
    // FinWait1,
    // FinWait2,
    // CloseWait,
    // Closing,
    LastAck,
    // TimeWait,
    Closed,
}

#[derive(Debug, Clone)]
pub struct Connection {
    // pub local_addr: SockAddr,
    // pub remote_addr: SockAddr,
    pub state: TcpState,
}

impl Connection {
    pub fn new_server() -> Self {
        Connection::new(TcpState::Listen)
    }

    // pub fn new_client(local_addr: SockAddr, remote_addr: SockAddr) -> Self {
    //     Connection::new(local_addr, remote_addr, TcpState::SynSent)
    // }

    fn new(state: TcpState) -> Self {
        Connection {
            // local_addr,
            // remote_addr,
            state,
        }
    }
}

pub struct ConnectionPool {
    connections: RwLock<HashMap<SockAddr, Arc<RwLock<Connection>>>>,
}

impl ConnectionPool {
    pub fn new() -> Self {
        ConnectionPool {
            connections: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, remote_addr: SockAddr) -> Arc<RwLock<Connection>> {
        if let Some(con) = self.connections.read().await.get(&remote_addr) {
            return con.clone();
        }

        let con = Arc::new(RwLock::new(Connection::new_server()));
        self.connections
            .write()
            .await
            .insert(remote_addr, con.clone());
        con
    }
}
