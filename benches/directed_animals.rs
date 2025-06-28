use criterion::{criterion_group, criterion_main, Criterion};
use xdd::{BDDFactory, DecisionDiagramFactory, NoMultiplicity, ZDDFactory};
use xdd_pictures::directed_animals::{count_directed_animals_by_memoization, count_directed_animals_xdd};

fn test_memoization() {
    let (_cache_size,values) = count_directed_animals_by_memoization(20);
    assert_eq!(vec![1,1,2,5,13,35,96,267,750,2123,6046,17303,49721,143365,414584,1201917,3492117,10165779,29643870,86574831], values);
}

fn test_xdd<F: DecisionDiagramFactory<u32,NoMultiplicity>>(_name:&str) {
    let (_original_size,_factory,_node,values) = count_directed_animals_xdd::<F>(20);
    assert_eq!(vec![1,1,2,5,13,35,96,267,750,2123,6046,17303,49721,143365,414584,1201917,3492117,10165779,29643870,86574831], values);
}


fn count_bdd() {
    test_xdd::<BDDFactory<u32,NoMultiplicity>>("BDD")
}

fn count_zdd() {
    test_xdd::<ZDDFactory<u32,NoMultiplicity>>("ZDD")
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count directed animals dynamic programming", |b| b.iter(|| test_memoization()));
    let mut slower = c.benchmark_group("count up to trionimoes");
    slower.sample_size(10);
    slower.bench_function("count directed animals BDD", |b| b.iter(|| count_bdd()));
    slower.bench_function("count directed animals ZDD", |b| b.iter(|| count_zdd()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
