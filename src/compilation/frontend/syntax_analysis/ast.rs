pub struct SourceFile<'source> {
    pub use_statements: Vec<UseStatement<'source>>,
}

pub struct UseStatement<'source> {
    pub target: UseTarget<'source>,
}

pub enum UseTarget<'source> {
    SingleIdentity(Identity<'source>),
}

pub struct Identity<'source> {
    pub payload: &'source str,
}
