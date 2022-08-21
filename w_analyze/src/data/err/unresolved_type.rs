struct UnresolvedTypeError<'a> {

}

impl<'a> AnalyzerError<'a> for UnresolvedTypeError<'a> {
    fn kind(&self) -> ErrKind {
        ErrKind::Error
    }
}
