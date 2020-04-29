//! Target JSON:
//!
//! ```text
//! let json = json!({
//!     "body": [{
//!         "expression": {
//!             "body": {
//!                 "end": 13,
//!                 "raw": "\"hello\"",
//!                 "start": 6,
//!                 "type": "Literal",
//!                 "value": "hello"
//!             },
//!             "start":0,
//!             "end":13,
//!             "params":[],
//!             "type": "ArrowFunctionExpression"
//!         },
//!         "start":0,
//!         "end":13,
//!         "type": "ExpressionStatement"
//!     }],
//!     "start":0,
//!     "end":13,
//!     "type": "Program"
//! });
//! ```
//!

use serde_json::{ json, to_value };
use ratel::parse;

use ratel::ast::node::Node;
use ratel::ast::statement::Statement;
use ratel::ast::expression::Expression;

use ambients_parser::ast::{ Expr, Exec };

// TODO: Error
fn js2amb<'input>(module: &ratel::Module) -> Result<Exec<'input>, ratel::error::Error> {
    println!("{:?}", module);

    fn traverse_body(body: ()) {
    }

    fn traverse_expression(expr: Node<Expression>) {
        match expr.item {
            Expression::Arrow(arrow) => match arrow.body {
                ratel::ast::expression::ArrowBody::Expression(expr) => traverse_expression(expr),
                ratel::ast::expression::ArrowBody::Block(block) => traverse_body(block.body)
            },
            Expression::This(_e) => (),
            Expression::Identifier(_e) => (),
            Expression::Void => (),
            Expression::Literal(_e) => (),
            Expression::Sequence(_e) => (),
            Expression::Array(_e) => (),
            Expression::Member(_e) => (),
            Expression::ComputedMember(_e) => (),
            Expression::MetaProperty(_e) => (),
            Expression::Call(_e) => (),
            Expression::Binary(_e) => (),
            Expression::Prefix(_e) => (),
            Expression::Postfix(_e) => (),
            Expression::Conditional(_e) => (),
            Expression::Template(_e) => (),
            Expression::TaggedTemplate(_e) => (),
            Expression::Spread(_e) => (),
            Expression::Object(_e) => (),
            Expression::Function(_e) => (),
            Expression::Class(_e) => ()
        }
    };

    let _: () = module.body().into_iter().map(|el| {
        match el.item {
            Statement::Expression(expr) => { traverse_expression(expr) },
            _ => ()
            // Statement::Declaration(_e) => (),
            // Statement::Return(_e) => (),
            // Statement::Break(_e) => (),
            // Statement::Continue(_e) => (),
            // Statement::Throw(_e) => (),
            // Statement::If(_e) => (),
            // Statement::While(_e) => (),
            // Statement::Do(_e) => (),
            // Statement::For(_e) => (),
            // Statement::ForIn(_e) => (),
            // Statement::ForOf(_e) => (),
            // Statement::Try(_e) => (),
            // Statement::Block(_e) => (),
            // Statement::Labeled(_e) => (),
            // Statement::Function(_e) => (),
            // Statement::Class(_e) => (),
            // Statement::Switch(_e) => (),
            // Statement::Empty => (),
            // Statement::Debugger => (),
        }
    }).collect();

    // item: Expression(
    //     Loc {
    //         start: 0,
    //         end: 13,
    //         item: Arrow(
    //             ArrowExpression {
    //                 params: [],
    //                 body: Expression(
    //                     Loc {
    //                         start: 6,
    //                         end: 13,
    //                         item: Literal(String("\"hello\""))
    //                     })
    //             })
    //      })

    // ArrowExpression = func[open_| ... ] | open func
    return Ok(Exec::Noop("x"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        let module = parse("() => \"hello\"").unwrap();
        let ambient_ast = js2amb(&module);

        let expected = Exec::Parallel(vec![
            Exec::Ambient("func", Box::new(Exec::Parallel(vec![
                Exec::Open_("*"),
                Exec::Ambient("string", Box::new(Exec::Noop("hello")))
            ]))),
            Exec::Open("func")
        ]);
        assert_eq!(format!("{:?}", ambient_ast), format!("{:?}", expected));
    }
}
