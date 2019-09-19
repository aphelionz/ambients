#![deny(missing_docs)]


//! Implementation of the [Ambient Protocol](https://github.com/ambientsprotocol/whitepaper)
//!
//! # Abstract
//!
//! > The decentralized web has shown great promise for highly available, massively distributed,
//! > open, censorship-free, privacy-preserving and non-exclusive application ecosystems. The
//! > technology landscape is, however, becoming fragmented due to a lack of collaboration and
//! > safe interoperability between platforms. This fragmentation exacerbates the uncertainty,
//! > risk and cost of decentralized application development and deployment. Furthermore,
//! > competing decentralized technology projects promote application programming models that
//! > are logically centralized, removing incentives for pursuing a fully decentralized web.
//!
//! > To counter this problem, we introduce the Ambients protocol to create and verifiably
//! > execute distributed programs in decentralized, peer-to-peer networks. The main contribution
//! > of the Ambients protocol is a process-algebraic specification of its purely functional,
//! > safe and composable programming model. Combined with a content-addressed and
//! > location-agnostic execution model, what we present is a novel protocol for
//! > distributed computation.
//!
//! > A deterministic evaluation of Ambients programs is guaranteed by a novel, strongly
//! > confluent, distributed rewrite system based on ambient calculus. Deterministic
//! > verification of the rewrite system, and its integrity and authenticity, are guaranteed
//! > by recording the execution and control flow of programs as Merkle-DAG-based, partially
//! > ordered, immutable event logs. Because of these guarantees, the Ambients protocol enables
//! > building and composing truly decentralized applications, databases and services, making
//! > safe interoperability and collaboration feasible within existing networks and platforms.
//!
//! # Testing 123
//! Objectives, per the whitepaper:
//! 1. compile original source code to an intermediate abstract syntax structure (usually as in
//! Abstract Syntax Tree)
//! 2. translate the intermediate structure to the computation primitives, distribution primitives
//! and computation abstractions of the Ambients protocol
//! 3. generate the bytecode executable from the primitives

mod prelude;

mod ambient;
mod primitives;
mod manifest;
mod keypair;
