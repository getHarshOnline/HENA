//! The swarm of Causality's research stack, an evolving swarm of MUON units that HENA hosts
//! would coordinate under shared budgets, with bounded differentiable memory and an emergent
//! hierarchy.
//!
//! One large model attends to everything at once; HENA asks whether many small units can
//! learn useful collective computation through sparse communication and limited memory. It
//! builds on MONAD's formal types, MUON's common unit interface and Reckon's record
//! protocol, and re-exports them today; Q-HENA adapts its collective design to quantum units.
//! A HENA host would own the swarm's store and recovery. Routing, scheduling and the swarm
//! store are the work in front of it.

pub use monad_kernel;
pub use muon_kernel;
pub use reckon_kernel;
