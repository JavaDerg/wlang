#![cfg_attr(debug_assertions, allow(dead_code))]

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use w_rcstr::{Origin, RcStr};
use w_tokenize::{tokenize, Kind, Span};

fn main() {
    let str = RcStr::new(include_str!("../../WIP_tests/mod_test/main.w").to_string(), Origin::Unknown);
    let file = Span::new(str.clone());
    let (_, tokens) = tokenize(file).unwrap();
    // let (_, parsed) = parse(TokenSpan::new(file, Rc::from(tokens))).unwrap();

    let mut colors = ColorGenerator::new();

    let mut rpb = Report::build(ReportKind::Error, "main.w", 0).with_code("oh snap");

    let blockc = colors.next();

    for tk in tokens.into_iter().skip(1) {
        if !matches!(tk.kind, Kind::Block(_)) {
            continue;
        }

        let span = tk.span;

        rpb = rpb.with_label(
            Label::new((
                "main.w",
                span.location_offset()..span.location_offset() + span.len(),
            ))
            .with_message("that's a block of tokens")
            .with_color(blockc),
        );
    }
    rpb.with_message("well something happened here")
        .with_note("your program sucks")
        .finish()
        .print(("main.w", Source::from(&*str)))
        .unwrap();
}
