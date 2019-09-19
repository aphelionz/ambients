//!  Deploying a program creates a root manifest file. This file contains the program "bytecode"
//!  and the public signing key of the deployer. The manifest is then signed by the deployer to
//!  prevent forging of program deployments. The manifest file is hashed and the hash of the
//!  manifest is the identifier of the program.
//!
//!  The identifier in turn is used to construct a program address.
//!  {
//!   program: 'zdpuAkfNT6xd5mC3Jk3ZNMGrjoqqRqSKTLjU...',
//!   name: 'hello-world',
//!   keys: '/amb/zdpuAuTSoDhKKgAfjJBRvWw4wSg5r6b3oW...',
//!   creator: {
//!     id: 'zdpuAwkLw7KAgXSEqduQQoyo9MrpkWrKDrKtBUg...',
//!     publicKey: '04c9680e7399c5d9589df2b62f32d568...'
//!   }
//!   signature: '30440220264d3bab838066d856087779af...',
//! }
//!

use crate::prelude::*;
use cid::Cid;
use crate::keypair::PublicKey;

#[derive(Debug)]
pub struct Address<'a> {
    protocol: &'a str,
    hash: &'a Cid,
}

/// The program address consists of the protocol prefix and the identifier, separated by /.
///
/// For example, if the manifest hashes to zdpuAwAdomEUPx54FZVLt33ZeGZ5VrJkTgLxQiUZNBwZ3kr7e,
/// the address of the program can be represented as (complete hash truncated for brevity):
///
/// > /amb/zdpuAwAdomEUPx54FZVLt33ZeGZ5VrJkTgLxQiUZNBwZ3...
impl<'a> Address<'a> {
    pub fn new (protocol: &'a str, hash: &'a Cid) -> Address<'a> {
        Address{ hash, protocol }
    }
}

#[derive(Debug)]
pub struct Creator<'a> {
    id: &'a Cid,
    public_key: &'a PublicKey
}

impl<'a> Creator<'a> {
    pub fn new (id: &'a Cid, public_key: &'a PublicKey) -> Creator<'a> {
        Creator{ id, public_key }
    }
}

#[derive(Debug)]
pub struct Manifest<'a> {
    program_cid: &'a Cid,
    name: &'a str,
    keys: Address<'a>,
    creator: Creator<'a>,
    signature: Vec<u8>,
}

impl<'a> Manifest<'a> {
    pub fn new (program_cid: &'a Cid, name: &'a str, keys: Address<'a>, creator: Creator<'a>, signature: Vec<u8>) -> Manifest<'a> {
        Manifest{
            program_cid: program_cid,
            name: name,
            keys: keys,
            creator: creator,
            signature: signature,
        }
    }
}

impl<'a> Display for Manifest<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "manifest")
    }
}
