use std::{fmt::Result, net::SocketAddr, time::Duration};
use tokio::{
    io,
    time::{error::Elapsed, sleep},
};

use crate::network::tcp_server::TcpServer;

#[derive(Debug, thiserror::Error)]
#[error("Acceptor error")]
pub enum Error {
    #[error("I/O {0}")]
    Io(#[from] io::Error),
    #[error("Timeout")]
    Timeout(#[from] Elapsed),
}

pub struct Broker {
    accept_thread_num: usize,
    max_connection_num: usize,
}

impl Broker {
    pub fn new(accept_thread_num: usize, max_connection_num: usize) -> Broker {
        // let (tx, rx) = flume::bounded(1);
        return Broker {
            accept_thread_num,
            max_connection_num,
        };
    }
    pub async fn start(&self) -> Result {
        // metrics init

        // network server start
        let ip: SocketAddr = "127.0.0.1:8768".parse().unwrap();
        let net_s = TcpServer::new(ip, self.accept_thread_num, self.max_connection_num);
        net_s.start().await;
        loop {
            sleep(Duration::from_secs(10)).await
        }
    }

    pub async fn stop(&self) -> Result {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common_base::runtime::create_runtime;
    use futures::executor::block_on;

    use super::Broker;

    #[test]
    fn start_broker() {
        let rt = create_runtime("text", 10);
        let guard = rt.enter();
        let b = Broker::new(10, 10);
        _ = block_on(b.start());
        drop(guard);
    }

    #[test]
    fn client() {}
}
