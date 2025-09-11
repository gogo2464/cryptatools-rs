use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cryptatools_core::utils::{convert::Encode,  alphabets::Alphabet};
use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::VigenereCoincidenceIndexGenerator;
 
fn benchmark_coincidence_index_generator() -> f64 {
    let printable_ascii_alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    let vcig = VigenereCoincidenceIndexGenerator::new(printable_ascii_alphabet.into());
    let ci = vcig.generate_coincidence_index_for_key_from_file(5, String::from("./cryptatools-core/data/text-corpus-for-statistics/gutenberg/austen-emma.txt"));
    ci
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("coincidence index generator 5", |b| b.iter(|| benchmark_coincidence_index_generator()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);