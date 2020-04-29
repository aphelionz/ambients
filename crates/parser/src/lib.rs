// #![deny(warnings)]

//! An LALR(1) Parser for Ambients Syntax. Work in Progress
//!
//! **Input:** a function or program written in Mobile Ambient syntax. For example:
//!
//! ```text
//! string_concat[
//!   in_ call.open call.(
//!     func[
//!       left[
//!         in_ arg.open arg.in string.in concat
//!       ]|
//!       right[
//!         in_ arg.open arg.in string.in concat
//!       ]|
//!       string[
//!         concat[in_ left|in_ right]|
//!         in_ left|in_ right
//!       ]|
//!       open_
//!     ]|
//!     open return.open_
//!   )
//! ]
//! ```
//!
//! **Output:** An Abstract Syntax Tree (AST) represented as Rust enum types, to be consumed
//! elsewhere.
//!
//! ## Tokens
//!
//! First the parser lexes the input using the following tokens:
//!
//! - **Pipes** `|` signify parallel computation (async)
//! - **Periods** `.` signify serial computation  (await)
//! - **Brackets** "[" and "]" signify ambient bounds of parallel or
//! - "]"
//!
//! ## Grammar
//!
//! Then, the parser uses these grammar rules to construct the AST
//!
//! ## Future Work:
//! - Implement types as described in the [Ambient Protocol Whitepaper](https://github.com/ambientsprotocol/whitepaper/blob/master/05-distributed-programs-as-ambients.md#types)

pub mod ast;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub ambients); // synthesized by LALRPOP

#[cfg(test)]
mod test {
    use pretty_assertions::{ assert_eq };
    use super::ambients::{ ExecutionParser as Parser };
    use super::ast::{ Exec::* };

    #[test]
    fn ambients_values() {
        let mut errors = Vec::new();

        let expr = Parser::new().parse(&mut errors, "a[]").unwrap();
        let expected = Noop("a");
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

        let expr = Parser::new().parse(&mut errors, "hello[]").unwrap();
        let expected = Noop("hello");
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    // #[test]
    // fn ambients_typed() {
    //     let expr = Parser::new().parse(&mut errors, "string[hello[]]").unwrap();
    //     let expected = STRING(Box::new(Noop("hello")));
    //     assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    // }

    #[test]
    fn ambients_parallel() {
        let mut errors = Vec::new();

        let expr = Parser::new().parse(&mut errors, "a[] | b[]").unwrap();

        let a = Noop("a");
        let b = Noop("b");

        let expected = Parallel(vec![a,b]);
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

        let expr = Parser::new().parse(&mut errors, "a[ b[] ] | c[]").unwrap();

        let b = Noop("b");
        let a = Ambient("a", Box::new(b));
        let c = Noop("c");
        let expected = Parallel(vec![a,c]);

        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    #[test]
    fn ambient_capabilities() {
        let mut errors = Vec::new();

        let expr = Parser::new().parse(&mut errors, "a[b[open_|c[]]|open b]").unwrap();
        let expected = Ambient("a", Box::new(Parallel(vec![
                    Ambient("b", Box::new(Parallel(vec![Open_("*"),Noop("c")]))),
                    Open("b")
        ])));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

       let expr = Parser::new().parse(&mut errors, "a[in b] | b[in_ a]").unwrap();
       let expected = Parallel(vec![
           Ambient("a", Box::new(In("b"))),
           Ambient("b", Box::new(In_("a")))
       ]);
       assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

       let expr = Parser::new().parse(&mut errors, "b[a[out b]|out_ a]").unwrap();
       let expected = Ambient("b", Box::new(Parallel(vec![
                   Ambient("a", Box::new(Out("b"))),
                   Out_("a")
       ])));
       assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

       let expr = Parser::new().parse(&mut errors, "a[b[open_|c[]]|open b]").unwrap();
       let expected = Ambient("a", Box::new(Parallel(vec![
                   Ambient("b", Box::new(Parallel(vec![Open_("*"), Noop("c")]))),
                   Open("b")
       ])));
       assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    #[test]
    fn ambient_paths() {
        let mut errors = Vec::new();

        let expr = Parser::new().parse(&mut errors, "a[in c] | b[in c] | c[in_ a.in_ b.in d] | d[in_ c]").unwrap();
        let expected = Parallel(vec![
            Ambient("a", Box::new(In("c"))),
            Ambient("b", Box::new(In("c"))),
            Ambient("c", Box::new(Serial(vec![In_("a"), In_("b"), In("d")]))),
            Ambient("d", Box::new(In_("c"))),
        ]);
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

        let expr = Parser::new().parse(&mut errors, "a[in b.in_ |b[]]").unwrap();
        let expected = Ambient("a", Box::new(Parallel(vec![
                Serial(vec![In("b"), In_("*")]),
                Noop("b")
        ])));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    // TODO: Func AST?
    #[test]
    fn ambient_func() {
        let mut errors = Vec::new();

        let expr = Parser::new().parse(&mut errors, "func[in_ x.open x.open_]").unwrap();
        let expected = Ambient("func", Box::new(
                Serial(vec![In_("x"), Open("x"), Open_("*")])
        ));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

        let expr = Parser::new().parse(&mut errors, "func[in_ x.open x.open_] | x[in func.open_|result[]] |open func").unwrap();
        let expected = Parallel(vec![
            Ambient("func", Box::new(Serial(vec![In_("x"), Open("x"), Open_("*")]))),
            Ambient("x", Box::new(Parallel(vec![
                        Serial(vec![In("func"), Open_("*")]),
                        Noop("result")
            ]))),
            Open("func")
        ]);
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    #[test]
    fn ambient_arg() {
        let mut errors = Vec::new();

        let expr = Parser::new().parse(&mut errors, "arg[in_ x.open x.in y.open_] | y[in_ arg.open arg.in func.open_]").unwrap();
        let expected = Parallel(vec![
            Ambient("arg", Box::new(Serial(vec![
                        In_("x"), Open("x"), In("y"), Open_("*")
            ]))),
            Ambient("y", Box::new(Serial(vec![
                        In_("arg"), Open("arg"), In("func"), Open_("*")
            ])))
        ]);
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

        let program = "
arg[in_ x.open x.in y.open_] | x[in arg.open_|input[]] |
y[in_ arg.open arg.in func.open_] |
func[in_ y.open y.open_]
";
        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Parallel(vec![
            Ambient("arg", Box::new(Serial(vec![In_("x"), Open("x"), In("y"), Open_("*")]))),
            Ambient("x", Box::new(Parallel(vec![Serial(vec![In("arg"), Open_("*")]), Noop("input")]))),
            Ambient("y", Box::new(Serial(vec![In_("arg"), Open("arg"), In("func"), Open_("*")]))),
            Ambient("func", Box::new(Serial(vec![In_("y"), Open("y"), Open_("*")])))
        ]);
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    #[test]
    fn ambient_function_expression() {
        let mut errors = Vec::new();

        let program = "
message[
  in func.open_|
  func[
    x[in_ arg.open arg.in message.open_]|
    message[in_ x.open x]|
    in_ arg.open_
  ]
] |
func[
  in_ message.open message.open func.open_|
  arg[
    in func.in x.open_|
    string[hello[]]
  ]
]|
open func
";
        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Parallel(vec![
            Ambient("message", Box::new(Parallel(vec![
                Serial(vec![In("func"), Open_("*")]),
                Ambient("func", Box::new(Parallel(vec![
                    Ambient("x", Box::new(
                        Serial(vec![In_("arg"), Open("arg"), In("message"), Open_("*")]))
                    ),
                    Ambient("message", Box::new(Serial(vec![In_("x"), Open("x")]))),
                    Serial(vec![In_("arg"), Open_("*")])
                ])))
            ]))),
            Ambient("func", Box::new(Parallel(vec![
                Serial(vec![In_("message"), Open("message"), Open("func"), Open_("*")]),
                Ambient("arg", Box::new(Parallel(vec![
                    Serial(vec![In("func"), In("x"), Open_("*")]),
                    Ambient("string", Box::new(Noop("hello")))
                ])))
            ]))),
            Open("func")
        ]);
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    #[test]
    fn ambient_call() {
        let mut errors = Vec::new();

        let program = "call[out x.in y.open_]";
        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Ambient("call", Box::new(Serial(vec![
            Out("x"), In("y"), Open_("*")
        ])));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

        let program = "
x[call[out x.in y.open_|payload[]] | out_ call] |
y[in_ call.open call]
";
        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Parallel(vec![
            Ambient("x", Box::new(Parallel(vec![
                Ambient("call", Box::new(Parallel(vec![
                    Serial(vec![Out("x"), In("y"), Open_("*")]),
                    Noop("payload")
                ]))),
                Out_("call")
            ]))),
            Ambient("y", Box::new(Serial(vec![In_("call"), Open("call")])))
        ]);
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    #[test]
    fn ambient_return() {
        let mut errors = Vec::new();

        let program = "return[open_.in x]";
        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Ambient("return", Box::new(Serial(vec![Open_("*"), In("x")])));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

        let program = "
x[
    call[out x.in y.open_|return[open_.in x]]|
    out_ call.in_ y
] |
y[in_ call.open call.open return]
";
        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Parallel(vec![
            Ambient("x", Box::new(Parallel(vec![
                Ambient("call", Box::new(Parallel(vec![
                    Serial(vec![Out("x"), In("y"), Open_("*")]),
                    Ambient("return", Box::new(Serial(vec![Open_("*"), In("x")])))
                ]))),
                Serial(vec![Out_("call"), In_("y")])
            ]))),
            Ambient("y", Box::new(Serial(vec![In_("call"), Open("call"), Open("return")])))
        ]);
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    #[test]
    fn ambient_monoid() {
        let mut errors = Vec::new();

        let program = "
string_concat[
  in_ call.open call.(
    func[
      left[
        in_ arg.open arg.in string.in concat
      ]|
      right[
        in_ arg.open arg.in string.in concat
      ]|
      string[
        concat[in_ left|in_ right]|
        in_ left|in_ right
      ]|
      open_
    ]|
    open return.open_
  )
]";
        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Ambient("string_concat", Box::new(
            Serial(vec![In_("call"), Open("call"), Group(Box::new(
                Parallel(vec![
                    Ambient("func", Box::new(Parallel(vec![
                        Ambient("left", Box::new(
                            Serial(vec![In_("arg"), Open("arg"), In("string"), In("concat")])
                        )),
                        Ambient("right", Box::new(
                            Serial(vec![In_("arg"), Open("arg"), In("string"), In("concat")])
                        )),
                        Ambient("string", Box::new(Parallel(vec![
                            Ambient("concat", Box::new(Parallel(vec![In_("left"), In_("right")]))),
                            In_("left"),
                            In_("right")
                        ]))),
                        Open_("*")
                    ]))),
                    Serial(vec![Open("return"), Open_("*")])
                ])))]
            )));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));

        let program = "
string[
  concat[
    left[string[a[]]]|
    right[string[b[]]]
  ]
]
";

        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let monad = Ambient("string", Box::new(
            Ambient("concat", Box::new(Parallel(vec![
                Ambient("left", Box::new(Ambient("string", Box::new(Noop("a"))))),
                Ambient("right", Box::new(Ambient("string", Box::new(Noop("b"))))),
            ])))
        ));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", monad));

        // TODO: Typo in paper!
        let program = "
string[
  concat[
    left[
      string[
        concat[
          left[string[a[]]]|
          right[string[b[]]]
        ]
      ]
    ]|
    right[string[c[]]]
  ]
]
";

        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Ambient("string", Box::new(
            Ambient("concat", Box::new(Parallel(vec![
                Ambient("left", Box::new(monad)),
                Ambient("right", Box::new(Ambient("string", Box::new(Noop("c")))))
            ])))));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }

    #[test]
    fn ambient_functors() {
        let mut errors = Vec::new();

        let program = "
identity[
  int[
    length[string[hello[]]]
  ]
]
";

        let expr = Parser::new().parse(&mut errors, program).unwrap();
        let expected = Ambient("identity", Box::new(
            Ambient("int", Box::new(
                Ambient("length", Box::new(
                    Ambient("string", Box::new(Noop("hello")))
                ))
            ))
        ));
        assert_eq!(&format!("{:?}", expr), &format!("{:?}", expected));
    }
}
