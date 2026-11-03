//! `flywheel-hosted`, the hosted control plane's implementation of the
//! control-plane interface (design Decision 1; core.2), and the one crate
//! that holds the runner's protocol with it: one JSON Schema per version,
//! under `protocol/`, and the runner's types for its messages (core.42).
//! It holds the protocol now. It implements no `ControlPlane` until the
//! change that builds the hosted implementation, and `planes.rs` lists it
//! from then.
