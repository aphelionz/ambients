//! The ambient is the fundamental computation abstraction in ambient calculus. It is a

use std::str::FromStr;
use cid::{ Cid, Codec, Version };
use ambients_parser::ast::Exec;
use ambients_parser::ambients::{ ExecutionParser, Token };

// use multihash::Hash;
// use crate::primitives::Target;
// use crate::manifest::{ Manifest, Address, Creator };
use crate::prelude::*;
// use crate::keypair::Keypair;

/// The ambient is the fundamental computation abstraction in ambient calculus. It is a
/// computation container, with well-defined boundaries that separate an ambient from other
/// ambients and isolate its internal computation from the outside world.
/// Being enclosed inside an ambient, the computation has an unambiguous execution context
/// and is not influenced by anything that happens outside the ambient. This means that the
/// ambient calculus can model systems where programs need to have deterministic outcomes,
/// regardless of their execution location, and can also track how and where programs are
/// being distributed during execution.
#[derive(Debug)]
pub struct Ambient<'a> {
    //cid: Cid,

    /// Ambients are addressed by name. Every ambient has a name, which is used to control and
    /// authorize all actions, access, and behavior of the ambient. Two distinct ambients can
    /// share a name, which is a powerful property when modeling non-deterministic behavior of
    /// parallel processes (we'll discuss why this is so powerful in the later chapters).
    /// Once an ambient is created, there's no way to change its name while it exists, which
    /// means that names are unforgeable. Because of this integrity guarantee, ambient names
    /// can carry deeper meaning than just being an identifier. For example, the Ambients
    /// protocol uses names to specify type information in data structures. Using the ROAM
    /// syntax, the ambient expression describing an ambient is simply:
    ///
    /// `a[]`
    ///
    /// Here `a` is a name of the ambient and the square brackets define the boundaries of the
    /// ambient, everything inside them is isolated from other ambients outside `a`.    /// name: &'a str,
    // name: String
    ast: Exec<'a>
}

impl<'a> FromStr for Ambient<'_> {
    type Err = std::fmt::Error;

    fn from_str(program: &str) -> Result<Self, Self::Err> {
        let mut errors = Vec::new();
        match ExecutionParser::new().parse(&mut errors, program) {
            Ok(ast) => {
                // let new_ast: Expr<'a> = ast.clone();
                Ok(Ambient { ast: Expr<'a> })
            }
            Err(_e) => Err(std::fmt::Error)
        }
    }
}

// unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//     ::std::slice::from_raw_parts(
//         (p as *const T) as *const u8,
//         ::std::mem::size_of::<T>(),
//     )
// }

//     let my_struct = MyStruct { id: 0, data: [1; 1024] };
//     let bytes: &[u8] = unsafe { any_as_u8_slice(&my_struct) };
//     println!("{:?}", bytes);

// fn hash<T>(content: T) -> Cid
// where T: Sized {
//     let bytes = unsafe { any_as_u8_slice(&content) };
//     let h = multihash::encode(multihash::Hash::SHA2256, bytes).unwrap();
//     Cid::new(Codec::DagCBOR, Version::V1, &h)
// }

// impl Ambient {
//     /// Creates a new Ambient
//     pub fn new(name: &str, program: &str) -> Ambient {
//         // TODO: Write access. Right now we'll either do * access or this key only.
//         // Currently doing the latter
//         // let keypair = Keypair::generate();
//         // let keypair_cid = hash(keypair.public());
//         // let keys = Address::new("amb", &keypair_cid);
//
//         // TODO: Proper creator
//         // let creator = Creator::new(&keypair_cid, keypair.public());
//         // let program_cid = hash(&program);
//
//         // let signature = keypair.secret().sign(program.as_bytes()).unwrap();
//         // let manifest = Manifest::new(&program_cid, name, keys, creator, signature);
//         // println!("{:?}", manifest);
//
//         // let manifest_cid = hash(&manifest);
//         // println!("{:?}", manifest_cid.to_string());
//         Ambient { ast: "".to_string() }
//     }
// }
//
// This exists simply so that an Ambient can be a ByteCode target as well as a Computation OpCode
// impl Target for Ambient {}

impl Display for Ambient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""ambient""#)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let program = "string[hello[]]";
        let ambient = Ambient::from_str(program).unwrap();
        println!("{}", ambient)
    }

    // fn ambient_new() {
    //     let program = "message[
    //                       in func.open_|
    //                       func[
    //                         x[in_ arg.open arg.in message.open_]|
    //                         message[in_ x.open x]|
    //                         in_ arg.open_
    //                       ]
    //                     ]";
    // }
}

