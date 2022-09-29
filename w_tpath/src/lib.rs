use either::Either;
use w_tokenize::Span;

pub struct TPath<'a> {
    pub rooted: Option<Either<(), Span<'a>>>,
    pub path: Vec<PathComponent<'a>>,
}

pub struct PathComponent<'a> {
    pub name: Either<Span<'a>, String>,
}
