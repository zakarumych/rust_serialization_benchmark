use alkahest::{deserialize, serialize_unchecked, Deserialize, Formula, Serialize};
use criterion::{black_box, Criterion};

pub fn bench<F, T, R>(name: &'static str, c: &mut Criterion, data: &T, read: R)
where
    F: Formula + ?Sized,
    for<'a> &'a T: Serialize<F>,
    for<'a> T: Deserialize<'a, F>,
    R: Fn(&[u8]),
{
    let mut group = c.benchmark_group(format!("{}/alkahest", name));

    const BUFFER_LEN: usize = 10_000_000;
    let mut buffer = vec![0u8; BUFFER_LEN];

    let mut size = 0;

    group.bench_function("serialize", |b| {
        b.iter(|| {
            size = serialize_unchecked::<F, &T>(black_box(data), &mut buffer);
            black_box(());
        })
    });

    group.bench_function("read", |b| {
        b.iter(|| {
            read(&buffer);
            black_box(());
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(deserialize::<F, T>(black_box(&buffer)).unwrap());
        })
    });

    crate::bench_size(name, "alkahest", &buffer[..size]);

    group.finish();
}
