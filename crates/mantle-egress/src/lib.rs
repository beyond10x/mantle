//! Loopback HTTP CONNECT proxy that is the only network path out of a sandboxed
//! session. It admits a fixed set of `host:port` destinations, resolves names itself
//! and refuses to dial non-public addresses, so every failure closes rather than opens.

pub mod addr;
pub mod allow;
pub mod head;
pub mod log;
pub mod proxy;

pub use allow::{Allowlist, DEFAULT_ALLOW, Destination};
pub use proxy::{Config, serve};
