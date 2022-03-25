use criterion::*;
use pack_num::u32::*;

fn bench_packs(c: &mut Criterion) {
    let mut g = c.benchmark_group("pack");

    g.bench_function("ifs", |b| {
        b.iter(|| {
            for i in 0..0x10000 {
                black_box(ifs_pack(black_box(i * i)));
            }
        })
    });

    g.bench_function("loop", |b| {
        b.iter(|| {
            for i in 0..0x10000 {
                black_box(loop_pack(black_box(i * i)));
            }
        })
    });
}

criterion_group!(benches, bench_packs);
criterion_main!(benches);
