use either::Either;
use w_tokenize::Span;

pub struct TPath {
    pub rooted: Option<Either<(), Span>>,
    pub path: Vec<PathComponent>,
}

pub struct PathComponent {
    pub name: Either<Span, String>,
}
