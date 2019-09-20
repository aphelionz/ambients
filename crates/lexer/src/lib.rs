#![allow(non_camel_case_types)]

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]

/// TODO:
/// Booleans
/// Integers
/// Floating-point numbers
/// Bytes
/// Characters
/// Strings
/// Tuples
/// Lists
enum Grammar {
    #[token = "["]
    A_,

    #[token = "]"]
    _A,

    #[token = "arg"]
    Arg,

    #[token = "call"]
    Call,

    #[token = "create"]
    Create,

    #[token = "in_"]
    CoIn,

    #[token = "open_"]
    CoOpen,

    #[token = "out_"]
    CoOut,

    #[token = "deploy"]
    Deploy,

    #[token = "func"]
    Func,

    #[token = "in "]
    In,

    #[token = "open"]
    Open,

    #[token = "out"]
    Out,

    #[token = "("]
    P_,

    #[token = ")"]
    _P,

    #[token = "|"]
    Parallel,

    #[token = "return"]
    Return,

    #[token = "string"]
    Str,

    #[token = "."]
    Wait,

    // Catchall for any identifiers. Can be
    #[regex = "[a-zA-Z_-]+"]
    Name,

    #[end]
    End,
    #[error]
    Error
}

#[cfg(test)]
mod tests {
    use super::*;
    use Grammar::*;

    fn test_lexer(program: &str, grammar: &[Grammar]) {
        let mut lexer = Grammar::lexer(program);
        for token in grammar.iter() {
            assert_eq!(lexer.token, *token);
            lexer.advance();
        }
    }

    /// Chapter 5

    #[test]
    fn simplest_example() {
        test_lexer(r#"a[]"#, &[ Name, A_, _A ]);
    }

    #[test]
    fn immobile_ambients() {
        test_lexer(r#"a[ b[] ] | c[]"#, &[
            Name, A_, Name, A_, _A, _A, Parallel, Name, A_, _A
        ]);
    }

    #[test]
    fn capabilities_in() {
        test_lexer(r#"a[in b] | b[in_ a]"#, &[
            Name, A_, In, Name, _A, Parallel, Name, A_, CoIn, Name, _A
        ]);
    }

    #[test]
    fn capabilities_open() {
        test_lexer(r#"a[b[open_|c[]]|open b]"#, &[
            Name, A_, Name, A_, CoOpen, Parallel, Name, A_, _A, _A, Parallel, Open, Name, _A
        ]);
    }

    #[test]
    fn func_ad_hoc() {
        test_lexer(r#"func[in_ x.open x.open_] | x[in func.open_|result[]] | open func"#, &[
            Func, A_, CoIn, Name, Wait, Open, Name, Wait, CoOpen, _A, Parallel,
            Name, A_, In, Func, Wait, CoOpen, Parallel, Name, A_, _A, _A, Parallel,
            Open, Func
        ]);
    }

    #[test]
    fn arg_example_1() {
        test_lexer(r#"arg[in_ x.open x.in y.open_] | y[in_ arg.open arg.in func.open_]"#, &[
            Arg, A_, CoIn, Name, Wait, Open, Name, Wait, In, Name, Wait, CoOpen, _A,
            Parallel,
            Name, A_, CoIn, Arg, Wait, Open, Arg, Wait, In, Func, Wait, CoOpen, _A
        ]);
    }

    #[test]
    fn arg_example_2() {
        test_lexer(r#"
            arg[in_ x.open x.in y.open_] | x[in arg.open_|input[]] |
            y[in_ arg.open arg.in func.open_] |
            func[in_ y.open y.open_]"#, &[
                Arg, A_, CoIn, Name, Wait, Open, Name, Wait, In, Name, Wait, CoOpen, _A, Parallel,
                Name, A_, In, Arg, Wait, CoOpen, Parallel, Name, A_, _A, _A, Parallel,
                Name, A_, CoIn, Arg, Wait, Open, Arg, Wait, In, Func, Wait, CoOpen, _A, Parallel,
                Func, A_, CoIn, Name, Wait, Open, Name, Wait, CoOpen, _A
            ])
    }

    #[test]
    fn message_example() {
        test_lexer(r#"
            message[
              in func.open_|
              func[
                x[in_ arg.open arg.in message.open_]|
                message[in_ x.open x]|
                in_ arg.open_
              ]
            ]"#,
        &[
            Name,
            A_,
                In, Func, Wait, CoOpen, Parallel,
                Func, A_,
                    Name, A_, CoIn, Arg, Wait, Open, Arg, Wait, In, Name, Wait, CoOpen, _A, Parallel,
                    Name, A_, CoIn, Name, Wait, Open, Name, _A, Parallel,
                    CoIn, Arg, Wait, CoOpen,
                _A,
            _A
        ]);

        test_lexer(r#"
            func[
                in_ message.open message.open func.open_|
                arg[
                    in func.in x.open_|
                    string[hello[]]
                ]
            ]| open func
        "#, &[
            Func, A_, CoIn, Name, Wait, Open, Name, Wait, Open, Func, Wait, CoOpen, Parallel,
            Arg, A_, In, Func, Wait, In, Name, Wait, CoOpen, Parallel, Str, A_, Name, A_, _A,
            _A, _A, _A, Parallel, Open, Func
        ]);
    }

    #[test]
    fn call_example() {
        test_lexer(r#"x[call[out x.in y.open_|payload[]] | out_ call] | y[in_ call.open call]"#, &[
            Name, A_, Call, A_, Out, Name, Wait, In, Name, Wait, CoOpen, Parallel, Name, A_,
            _A, _A, Parallel, CoOut, Call, _A, Parallel, Name, A_, CoIn, Call, Wait, Open,
            Call, _A
        ]);
    }

    #[test]
    fn return_example() {
        test_lexer(r#"
            x[
                call[out x.in y.open_|return[open_.in x]]|
                out_ call.in_ y
            ] |
            y[in_ call.open call.open return]
        "#, &[
            Name, A_, Call, A_, Out, Name, Wait, In, Name, Wait, CoOpen, Parallel, Return, A_,
            CoOpen, Wait, In, Name, _A, _A, Parallel, CoOut, Call, Wait, CoIn, Name, _A, Parallel,
            Name, A_, CoIn, Call, Wait, Open, Call, Wait, Open, Return, _A
        ])
    }

    #[test]
    fn string_concat() {
        test_lexer(r#"
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
            ]
        "#, &[
            Name, A_,
                CoIn, Call, Wait, Open, Call, Wait, P_,
                    Func, A_,
                        Name, A_,
                            CoIn, Arg, Wait, Open, Arg, Wait, In, Str, Wait, In, Name,
                        _A, Parallel,
                        Name, A_,
                            CoIn, Arg, Wait, Open, Arg, Wait, In, Str, Wait, In, Name,
                        _A, Parallel,
                        Str, A_,
                            Name, A_, CoIn, Name, Parallel, CoIn, Name, _A, Parallel,
                            CoIn, Name, Parallel, CoIn, Name,
                        _A, Parallel,
                        CoOpen,
                    _A, Parallel,
                    Open, Return, Wait, CoOpen,
                _P,
            _A
        ])
    }
}
