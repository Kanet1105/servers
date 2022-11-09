use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bytes::{BytesMut, BufMut};

fn serialize_1(buffer: &mut BytesMut, data: &[u8]) {
    buffer.put_slice(data);
}

fn serialize_2(buffer: &mut BytesMut, data: u64) {
    buffer.put_u64(data);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut buffer_1 = BytesMut::new();
    let mut buffer_2 = BytesMut::new();
    let data: u64 = 125243262;
    c.bench_function("serialize_1", |b| b.iter(|| serialize_1(&mut buffer_1, &data.to_be_bytes())));
    c.bench_function("serialize_2", |b| b.iter(|| serialize_2(&mut buffer_2, data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
