/*!
r-cat library crate.

This file exposes the CLI definitions (re-exporting the `cli` module)
and the `net` modules (tcp, udp) so integration tests and other crates
can access `r_cat::cli::Args` and `r_cat::net::{tcp, udp}`.
*/

pub mod cli;
pub use cli::Args;

pub mod net;
pub use net::{tcp, udp};
