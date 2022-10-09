use std::net::SocketAddr;
use std::str::FromStr;

use crate::addr::Addr;

use super::ProxiedAddr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display, From)]
#[display(inner)]
pub enum UniversalAddr<A: Addr = SocketAddr> {
    #[from]
    Proxied(ProxiedAddr<A>),

    #[from]
    Direct(SocketAddr),
}

impl<A: Addr> Addr for UniversalAddr<A> {
    fn port(&self) -> u16 {
        match self {
            UniversalAddr::Proxied(addr) => addr.port(),
            UniversalAddr::Direct(socket) => socket.port(),
        }
    }
}

impl<A: Addr> From<&UniversalAddr<A>> for SocketAddr {
    fn from(addr: &UniversalAddr<A>) -> Self {
        match addr {
            UniversalAddr::Proxied(proxied) => proxied.into(),
            UniversalAddr::Direct(socket_addr) => *socket_addr,
        }
    }
}

impl<A: Addr> From<UniversalAddr<A>> for SocketAddr {
    fn from(addr: UniversalAddr<A>) -> Self {
        match addr {
            UniversalAddr::Proxied(proxied) => proxied.into(),
            UniversalAddr::Direct(socket_addr) => socket_addr,
        }
    }
}

impl<A: Addr> FromStr for UniversalAddr<A> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
