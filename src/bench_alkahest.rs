use alkahest::{deserialize, serialize, Deserialize, Formula, Serialize};
use criterion::{black_box, Criterion};

pub fn bench<T, A, R>(name: &'static str, c: &mut Criterion, data: &T, access: A, read: R)
where
    T: Formula,
    for<'de> T: Deserialize<'de, T>,
    for<'a> &'a T: Serialize<T>,
    A: Fn(&[u8]),
    R: Fn(&[u8]),
{
    let mut group = c.benchmark_group(format!("{}/alkahest", name));

    const BUFFER_LEN: usize = 10_000_000;
    let mut buffer = vec![0u8; BUFFER_LEN];

    let mut size = 0;

    group.bench_function("serialize", |b| {
        b.iter(|| {
            size = serialize::<T, &T>(black_box(data), &mut buffer).unwrap();
            black_box(());
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(deserialize::<T, T>(black_box(&buffer)).unwrap());
        })
    });

    group.bench_function("access (validated)", |b| {
        b.iter(|| {
            access(&buffer);
            black_box(());
        })
    });

    group.bench_function("read (validated)", |b| {
        b.iter(|| {
            read(&buffer);
            black_box(());
        })
    });

    crate::bench_size(name, "alkahest", &buffer);

    group.finish();
}
