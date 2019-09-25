#[derive(Debug, Clone)]
pub enum Exec<'input> {
    Parallel(Vec<Exec<'input>>),
    Serial(Vec<Exec<'input>>),
    Noop(ID<'input>),
    Ambient(ID<'input>, Box<Exec<'input>>),
    Group(Box<Exec<'input>>),

    Open(ID<'input>),
    Open_(ID<'input>),
    In(ID<'input>),
    In_(ID<'input>),
    Out(ID<'input>),
    Out_(ID<'input>),

    // STRING(Box<Exec<'input>>)
}

#[derive(Debug, Clone)]
pub enum Expr<'input> {
    // Capabilities and Co-Capabilities
    Create(ID<'input>),
    Deploy(ID<'input>),
}

// "Atom" types are just basic Rust types
type ID<'input> = &'input str;
