#![allow(non_camel_case_types)]

//! ## Protocol Primitives
//!
//! Not all mobile ambients seem to be translatable to values and functions in a way that makes sense for programs. For example, what kind of function would a mobile ambient `a[in b]` represent, or what kind of value does `hello[]` represent? We realize that to model actual values and functions and to compose them to full-blown programs, there needs be some transformation between the calculus and features present in programming models, like function arguments, evaluation scopes, data types etc. The Ambients protocol introduces a set of _protocol primitives_ which provide a translation from programming constructs to an _encoding_ of a program as ROAM expressions.
//!
//! In the Ambients protocol, [_values_](#values) are the elementary construct to which all computations reduce. In other words, the result of every computation in Ambients, is a value. The computations are represented by _protocol primitives_ which  consist of [_computation primitives_](#computation-primitives) and [_distribution primitives_](#distribution-primitives).
//!
//! _Protocol primitives_ are ambients which have special purpose in all Ambients programs. They are designed to assist remote and local computations with eventually converging to their final result. We define the following four primitives to encode programs as ambients:
//!
//! - [`func`](#computation-context-func)-ambient, which creates a distributable computational context for function evaluation
//! - [`arg`](#computation-parameter-arg)-ambient, which transfers values and functions between computational contexts
//! - [`call`](#request-computation-call)-ambient, which initiates function evaluation sequences
//! - [`return`](#return-computation-return)-ambient, which redirects remote or local code to a computational context where evaluation happens
//!
//! Next, we'll define what values are in Ambients as they define the ultimate result of all protocol primitives - to encode a distributed program as a function that reduces to a value. We will then continue to define the protocol primitives.
use std::{ fmt, fmt::Debug as Debug, fmt::Display as Display };

use crate::ambient::Ambient;

/// A tuple containing an OpCode and a Target
///
/// It is important to ensure that programs deployed to the network keep the information
/// hidden from the computation participants who don't need to access it. This is one of the
/// key properties of the execution model and one of the requirements is that programs can be
/// sliced into their parallel sub-parts, so that only a minimal part of the program is
/// exposed to the other participants. The compilation model, and its implementation, the
/// compiler, satisfies this requirement by producing a bytecode representation for every
/// unique ambient and their nested ambients as the compiler output.
///
/// The program instructions, each parallel sub-part of the program (a "slice"), and their
/// call-order, are represented as a DAG and saved to a content-addressed storage, as a
/// Merkle-DAG, giving each program and their sub-parts a unique hash. Using this hash,
/// the program can be fetched from the network and referenced by the programs.
/// Storing the bytecode as a Merkle-DAG, we can be assured that upon fetching the program
/// from the network, the bytecode hasn't been tampered with. By sharing the hash of the
/// bytecode of the program, the program can be discovered in the network and
/// included in other programs as a dependency.
// #[derive(Debug)]
//struct Instruction<'a, O, T> where O: OpCode, T: Target + 'a {
//    opcode: O,
//    target: &'a T
//}

/// Marker trait for the Capability, Computation, and Distribution enums, capturing the type of
/// the instruction to be executed.
///
/// We first define a set of opcodes for the events specific to the execution model
/// and the opcodes for the Robust Ambient calculus terms, the capabilities and co-capabilities:
///
/// ```text
/// 0: create
/// 1: deploy
/// 2: in
/// 3: in_
/// 4: out
/// 5: out_
/// 6: open
/// 7: open_
/// ```
///
/// We then define opcodes for the computation and distribution primitives of the protocol:
///
/// ```text
/// 0: func
/// 1: call
/// 2: arg
/// 3: return
/// ```
trait OpCode {}

/// Events specific to the execution model: `create`, `deploy`, `in`, `in_`, `out`, `out_`, `open`,
/// `open_`.
///
/// The opcodes capture the type of the instruction to be executed. We first
/// define a set of opcodes for the events specfic to the execution model
/// and the opcodes for the Robust Ambient calculus terms, the
/// capabilities and co-capabilities:
// #[derive(Debug)]
// enum Capability {
//     create = 0,
//     deploy = 1,
//     r#in = 2,
//     in_ = 3,
//     out = 4,
//     out_ = 5,
//     open = 6,
//     open_ = 7
// }
//
// impl OpCode for Capability {}
//
// impl Display for Capability {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Capability::create => write!(f, "0 create"),
//             Capability::deploy => write!(f, "1 deploy"),
//             Capability::r#in => write!(f, "2 in"),
//             Capability::in_ => write!(f, "3 in_"),
//             Capability::out => write!(f, "4 out"),
//             Capability::out_ => write!(f, "5 out_"),
//             Capability::open => write!(f, "6 open"),
//             Capability::open_ => write!(f, "7 open_")
//         }
//     }
// }

/// Evaluate functions with `func` and pass parameters via `arg`.
///
/// The Ambients Programming Model ensures that all programs will terminate, which means that
/// their eventual end result is an immutable value. When encoding programs as Ambients, the
/// final result is represented by an immobile ambient. However, being distributed and possibly
/// highly parallel, the Ambients programs have inherent, unavoidable non-determinism, which
/// becomes a problem when the programming model requires that programs have deterministic
/// outputs. At the same time, programs are expected to be composable. In order to have safe,
/// composable and deterministic encoding and evaluation of programs, the Ambients protocol
/// defines two primitives called func and arg.
/// ### Function Expressions With `func` and `arg`
///
/// With just `func` and `arg` primitives, we can express all pure functions. The general rule
/// for defining function expression is to compose the function declaration with the function
/// evaluation. This simply means composing two `func`s - the _declaration-site_ which declares
/// the parameter and the _call-site_ which passes the argument - and an `arg` to bind the
/// argument to a parameter between the two `func`s.
///
/// For example, a function expression `message("hello")` is a composition of the function
/// definition `message(x)` which declares the parameter `x`
///
/// ```text
/// message[
///   in func.open_|
///   func[
///     x[in_ arg.open arg.in message.open_]|
///     message[in_ x.open x]|
///     in_ arg.open_
///   ]
/// ]
/// ```
///
/// and the function evaluation which passes the value `string[hello[]]` as an argument:
///
/// ```text
/// func[
///   in_ message.open message.open func.open_|
///   arg[
///     in func.in x.open_|
///     string[hello[]]
///   ]
/// ]|
/// open func
/// ```
///
/// Composing these together reduces the whole program to a value:
///
/// ```text
/// message[string[hello[]]]
/// ```
///
/// To analyze the function encodings in general, let's categorize the encodable functions by their return type and the number of parameters they have.
///
/// Functions that expect zero parameters are _constant functions_, which means that they always evaluate to the same result. Constant functions returning values are used when values need to be transformed to a function-form, e.g. as arguments to generic functions. Constant functions that return functions are the basis for [locally evaluated functions](#evaluation-strategies). For example, JavaScript function `() => "hello"` can be encoded simply as a composition of the function definition and an evaluation without argument binding:
///
/// ```text
/// func[
///   open_|
///   string[hello[]]
/// ]|
/// open func
/// ```
///
/// Functions that expect more than zero parameters are generally ones that do more computation. Single-argument functions that return values are necessary for expressing transformations from input to output value. Single-argument functions that return functions enable [_currying_](https://en.wikipedia.org/wiki/Currying), which is how functions with more than one argument can be expressed.
// #[derive(Debug)]
// enum Computation {
//     /// The `func` primitive defines a computational context for function evaluation. It
//     /// establishes an evaluation scope and its behavior is similar to the widely established
//     /// concept of function scoping.
//     ///
//     /// Having a designated primitive for an evaluation scope allows Ambients programs to define
//     /// parallel and sequential control flows that always converge to a deterministic value.
//     /// As an ambient, func can be safely distributed as it isolates the computation inside it
//     /// from other ambients, i.e. it preserves the integrity of its internal computation. In
//     /// practice, this means that the runtime environment can use different evaluation strategies
//     /// to decide whether a computation is evaluated locally or remotely (i.e. composed as either
//     /// nested or parallel funcs) or as a mix of both, and to track and verify the state of the
//     /// computation with less complexity, in real time.
//     ///
//     /// Informally, a func for a function x is defined as:
//     ///
//     ///  `func[in_ x.open x.open_]`
//     ///
//     /// Here, the func primitive defines three, logically sequential phases:
//     ///
//     /// Initiate the evaluation scope by allowing computation x to enter it with in_ x.
//     /// This scope creates a safe addressing space for x, protected and isolated from other
//     /// parallel computations outside func.
//     ///
//     /// Evaluate the computation x by opening it with open x.
//     ///
//     /// Reveal the computation result to the outside by allowing itself to be opened with open_.
//     /// The func above is fully reduced by the following steps (result[] representing an ad hoc computation result of x):
//     ///
//     /// ```text
//     ///  func[in_ x.open x.open_] | x[in func.open_|result[]] |
//     ///  open func
//     ///→ func[x[open_|result[]] | open x.open_] | open func
//     ///→ func[result[] | open_]  | open func
//     ///→ result[]
//     /// ```
//     func = 0,
//     /// The `arg` primitive is used with func to transfer values and functions between ambients before their evaluation. This is how the protocol models function expressions with arguments. The arg primitive defines the argument binding procedure between parameters that are declared by functions, and arguments that are passed to functions in function expressions.
//     ///
//     /// Informally, arg acts as a container for an argument x to transfer it to a func to be evaluated as parameter y:
//     ///
//     /// arg[in_ x.open x.in y.open_] |
//     /// y[in_ arg.open arg.in func.open_]
//     /// Here, the arg primitive defines the binding between the argument x and the parameter y in three, logically sequential phases:
//     ///
//     /// The arg waits for an argument x, then evaluates it, and finally moves inside the parameter y to be evaluated.
//     /// The parameter y waits for an arg, then evaluates it, and finally moves inside a func to be evaluated.
//     /// When the parameter y is opened inside func, it will evaluate to whatever value or function the argument x originally contained.
//     /// The composite expression above is fully reduced to a func ready for evaluation by the following steps (where input[] represent an ad hoc value of input x):
//     ///
//     ///   arg[in_ x.open x.in y.open_] | x[in arg.open_|input[]] |
//     ///   y[in_ arg.open arg.in func.open_] |
//     ///   func[in_ y.open y.open_]
//     /// → arg[open x.in y.open_ | x[open_|input[]] ] |
//     ///   y[in_ arg.open arg.in func.open_] |
//     ///   func[in_ y.open y.open_]
//     /// → arg[in y.open_|input[]] |
//     ///   y[in_ arg.open arg.in func.open_] |
//     ///   func[in_ y.open y.open_]
//     /// → y[open arg.in func.open_|arg[open_|input[]]] |
//     ///   func[in_ y.open y.open_]
//     /// → y[in func.open_|input[]] |
//     ///   func[in_ y.open y.open_]
//     /// → func[open y.open_ | y[open_|input[]]]
//     /// → func[open_ | input[]]
//     arg = 2,
// }
//
// /// Request computation with `call` and return computation with `return`.
// ///
// /// The computation primitives encode distributed programs as ROAM expressions representing
// /// functions. In addition to function definition and evaluation, distribution of the functions
// /// is crucial for the protocol. The Ambients protocol defines two primitives, `call` and
// /// `return`, for controlled, safe, and modular distribution of programs and data.
// #[derive(Debug)]
// enum Distribution {
//     /// The `call` primitive allows functions to call other functions which may be local or remote. Therefore, invoking a `call` can be seen as a starting point for distributing computational workload in any program.
//     ///
//     /// Informally, a function `x`, which calls function `y`, creates a `call` primitive defined as:
//     ///
//     /// ```text
//     /// call[out x.in y.open_]
//     /// ```
//     ///
//     /// Here, the `call` primitive has three sequential phases:
//     ///
//     /// 1. Exit function `x` with `out x`.
//     /// 2. Enter function `y` with `in y`.
//     /// 3. Reveal the _call payload_ to the function `y` by allowing `call` to be opened with `open_`.
//     ///
//     /// The `call` above is fully reduced by the following steps (where `payload[]` represents an ad hoc computation payload):
//     ///
//     /// ```text
//     ///   x[call[out x.in y.open_|payload[]] | out_ call] |
//     ///   y[in_ call.open call]
//     /// → x[] | call[in y.open_|payload[]] | y[in_ call.open call]
//     /// → x[] | y[call[open_|payload[]] | open call]
//     /// → x[] | y[payload[]]
//     /// ```
//     call = 1,
//     /// The purpose of the `return` primitive is to include the needed instructions in a `call` to move the program control back to the _caller_, along with a result or remaining computation. Moving the control and the result back to the caller makes the evaluation of remote function possible as it's similar to the programming concept of replacing a function expression with a [return value](https://en.wikipedia.org/wiki/Return_statement). The `return` primitive also enables declaration of functions in ROAM expression in a way that decouples them from any potential caller.
//     ///
//     /// Informally, a `return` which moves the control back to a function `x` is defined as:
//     ///
//     /// ```text
//     /// return[open_.in x]
//     /// ```
//     ///
//     /// The [previous example](#request-computation-call), where the `payload` is replaced with a `return` primitive, is fully reduced by the following steps:
//     ///
//     /// ```text
//     ///   x[
//     ///     call[out x.in y.open_|return[open_.in x]]|
//     ///     out_ call.in_ y
//     ///   ] |
//     ///   y[in_ call.open call.open return]
//     /// → x[in_ y] | call[in y.open_|return[open_.in x]] |
//     ///   y[in_ call.open call.open return]
//     /// → x[in_ y] |
//     ///   y[call[open_|return[open_.in x]]|open call.open return]
//     /// → x[in_ y] | y[return[open_.in x]|open return]
//     /// → x[in_ y] | y[in x]
//     /// → x[y[]]
//     /// ```
//     ///
//     /// Here, the usage of `return` within `call` defines a logically sequential sequence, in addition to `call` handling:
//     ///
//     /// 1. After sending out the `call` to `y`, function `x` allows `y` to enter with `in_ y`.
//     /// 2. After opening the `call`, function `y` opens the `return` primitive, which reveals the `in x` capability, making `y` to enter `x` for further evaluation. Due to the sequential `out_ call.in_ y` definition, any `y` will be authorized to enter `x` only once and after the `call` to `y` has moved out of `x`.
//     ///
//     /// Because of this mechanism, the function `y` is unaware and fully decoupled from the caller `x` during the whole sequence, until it processes the `call` and the nested `return` and adopts the `in x` capability that redirects it back to the call-site `x`. Making the function `y` itself move, instead of creating some transient ambient representing a "return value", is a deliberate desig choice which enables a variety of [distributed evaluation strategies](#evaluation-strategies).
//     r#return = 3
// }
//
// impl OpCode for Distribution {}
//
// impl OpCode for Computation {}
//
// impl Display for Computation {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Computation::func => write!(f, "0 func"),
//             Computation::arg => write!(f, "2 arg"),
//         }
//     }
// }
//
// impl Display for Distribution {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Distribution::r#return => write!(f, "3 return"),
//             Distribution::call => write!(f, "1 call")
//         }
//     }
// }
//
// /// We continue with the denition that the target in the
// /// (<opcode>, <target>) tuple is either the opcode for the primitive or
// /// the name of the target ambient. For the co-capability open_ , the target
// /// is not used - instead, always use 0 as the target opcode. That is, open_
// /// compiles to (7, 0) .
//
// pub trait Target {
// }
//
// impl Target for Computation { }
// impl Target for Distribution { }
//
// impl<'a, O, T> Instruction<'a, O, T>
// where O: OpCode,
//       T: Target + 'a {
//     fn new (opcode: O, target: &T) -> Instruction<O, T> {
//         Instruction{ opcode, target }
//     }
// }
//
// impl<'a, O, T> Display for Instruction<'a, O, T>
// where O: OpCode + Display,
//       T: Target + Display + 'a {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", &self.opcode, &self.target)
//     }
// }


#[cfg(test)]
mod tests {
    //! What happens if I document the tests module?

    use super::*;

    // #[test]
    // fn instruction_display() {
    //     let ambient = Ambient::new("beep-boop", "beep[boop[]]");
    //     let instruction = Instruction::new(Capability::create, &ambient);
    //     assert_eq!(r#"(0 create, "ambient")"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Capability::deploy, &ambient);
    //     assert_eq!(r#"(1 deploy, "ambient")"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Capability::r#in, &ambient);
    //     assert_eq!(r#"(2 in, "ambient")"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Capability::in_, &ambient);
    //     assert_eq!(r#"(3 in_, "ambient")"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Capability::out, &ambient);
    //     assert_eq!(r#"(4 out, "ambient")"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Capability::out_, &ambient);
    //     assert_eq!(r#"(5 out_, "ambient")"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Capability::open, &ambient);
    //     assert_eq!(r#"(6 open, "ambient")"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Capability::open_, &ambient);
    //     assert_eq!(r#"(7 open_, "ambient")"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Computation::func, &Computation::func);
    //     assert_eq!(r#"(0 func, 0 func)"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Computation::arg, &Computation::arg);
    //     assert_eq!(r#"(2 arg, 2 arg)"#, format!("{}", instruction));
    //     let instruction = Instruction{ opcode: Distribution::r#return, target: &Distribution::r#return };
    //     assert_eq!(r#"(3 return, 3 return)"#, format!("{}", instruction));
    //     let instruction = Instruction::new(Distribution::call, &Distribution::call);
    //     assert_eq!(r#"(1 call, 1 call)"#, format!("{}", instruction));
    // }
}
