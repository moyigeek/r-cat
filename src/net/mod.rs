/*!
r-cat/src/net/mod.rs

Networking module for r-cat. This module exposes the TCP and UDP helpers
as submodules so callers can use `r_cat::net::tcp` and `r_cat::net::udp`.

The actual implementations live in `tcp.rs` and `udp.rs` within the same
directory.
*/

pub mod tcp;
pub mod udp;
