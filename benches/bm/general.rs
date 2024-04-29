use divan::Bencher;

const BOARDS: [&str; 2] = [
    "∙ ∙ ∙\n  ×  \n∙ ∙×∙\n  ×  \n∙ ∙ ∙",
    "∙ ∙ ∙\n     \n∙×∙ ∙\n     \n∙ ∙ ∙",
];

#[divan::bench]
pub fn map_fold(bencher: Bencher) {
    bencher.with_inputs(|| BOARDS).bench_refs(|[a, b]| {
        a.lines()
            .zip(b.lines())
            .map(|(x, y)| format!("{x} ║ {y}\n"))
            .fold(String::new(), |a, b| a + &b);
    })
}
#[divan::bench]
pub fn fold(bencher: Bencher) {
    bencher.with_inputs(|| BOARDS).bench_refs(|[a, b]| {
        a.lines()
            .zip(b.lines())
            .fold(String::new(), |a, (b, c)| format!("{a}{b} ║ {c}\n"));
    })
}
