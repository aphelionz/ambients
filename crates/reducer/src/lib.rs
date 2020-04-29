use ambients_parser::ast::Exec;

fn get_children<'input>(ast: &'input Exec<'input>) -> &'input Exec<'input> {
    match ast {
        Exec::Ambient(_e, c) => (),
        Exec::Noop(_e) => (),
        Exec::Parallel(_e) => (),
        Exec::Serial(_e) => (),
        Exec::Group(_e) => (),

        Exec::Open(_e) => (),
        Exec::Open_(_e) => (),
        Exec::In(_e) => (),
        Exec::In_(_e) => (),
        Exec::Out(_e) => (),
        Exec::Out_(_e) => ()
    };
    ast
}

fn create_transition_tree_recursive<'input>(ast: &Exec<'input>) {
    let children = get_children(&ast);
//   List.fold_left((res, acc: ambient) => {
//     let child = createTransitionTreeRecursive(acc);
//     let updated = _updatedWith(child, getChildren(res)) |> updateChildren(ambient);
//     let transition = createTransition(acc, ambient);
//     switch transition {
//     | Some(a) => updateTransitions(updated, [a, ...getTransitions(ambient)])
//     | None => updated
//     };
//   }, ambient, children);
    ()
}

fn can_reduce(tree: ()) -> bool {
    false
}


fn apply_transitions_recursive(tree: ()) {
}

fn reduce_fully<'input>(ast: Exec<'input>) -> Exec<'input>{
    println!("{:?}", ast);
    let transition_tree = create_transition_tree_recursive(&ast);
    match can_reduce(transition_tree) {
        true => {
            let transition_tree = apply_transitions_recursive(transition_tree);
            reduce_fully(ast)
        },
        false => ast
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    use ambients_parser::ast::Exec::{ Parallel, Ambient, Serial, Open, In, Open_, In_, Noop };

    #[test]
    fn it_works() {
        // a[in c] | b[in c] | c[in_ a.in_ b.in d] | d[in_ c]
        // →         b[in c] | c[in_ b.in d | a[]] | d[in_ c]
        // →                   c[in d | b[] | a[]] | d[in_ c]
        // →                                  d[c[b[] | a[]]]
        let ast = Parallel(vec![
            Ambient("a", Box::new(Serial(vec![In("c")]))),
            Ambient("b", Box::new(Serial(vec![In("c")]))),
            Ambient("y", Box::new(Serial(vec![In_("a"), In_("b"), In("d")]))),
            Ambient("d", Box::new(Serial(vec![In_("c")]))),
        ]);

        let reduced = reduce_fully(ast);
        let expected = Ambient("d", Box::new(
            Ambient("c", Box::new(Parallel(vec![Noop("b"), Noop("a")])))
        ));
        assert_eq!(format!("{:?}", reduced), format!("{:?}", expected));
    }
}
