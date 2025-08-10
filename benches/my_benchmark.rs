use criterion::{black_box, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use serde_json::Value as SerdeValue;


fn bench_parsers(c: &mut Criterion) {
    // load or embed sample JSON inputs
    // put test JSON files under tests/data or similar and use include_str!()
    let inputs: &[(&str, &str)] = &[
        ("small", include_str!("../tests/data/small.json")),
        ("medium", include_str!("../tests/data/medium.json")),
        ("large", include_str!("../tests/data/large.json")),
    ];

    let mut group = c.benchmark_group("json_parsers");
    group.sample_size(60); // increase for more stable results (default 100). adjust as you like.

    for (name, s) in inputs {
        group.throughput(Throughput::Bytes(s.len() as u64));

        group.bench_with_input(BenchmarkId::new("my_parser", name), s, |b, s| {
            b.iter(|| {
                // black_box prevents the compiler from optimizing away the call/argument
                let _ = arjer::parse(black_box(s)).unwrap();
            })
        });

        group.bench_with_input(BenchmarkId::new("serde_json", name), s, |b, s| {
            b.iter(|| {
                let _: SerdeValue = serde_json::from_str(black_box(s)).unwrap();
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_parsers);
criterion_main!(benches);
