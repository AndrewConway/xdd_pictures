
use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::Display;
use std::mem;
use xdd::{BDDFactory, DecisionDiagramFactory, NoMultiplicity, ZDDFactory};
use xdd::generating_function::GeneratingFunctionWithMultiplicity;
use chessboard_tiling_pictures::chessboard::{setup_chessboard_tiled_with_dominoes, setup_chessboard_tiled_with_up_to_trionimoes};
use chessboard_tiling_pictures::tiling_problem::TilingProblem;

/// Count using a decision diagram, given a creator function for the factory taking the number of variables.
fn count_tiling<F: DecisionDiagramFactory<u32, NoMultiplicity>,G:GeneratingFunctionWithMultiplicity<NoMultiplicity>+Display>(problem:TilingProblem<[i32;2]>) -> G {
    let (factory ,solution) = problem.find_tiling_solution::<F>();
    //let original_len = factory.len();
    //let renamer = factory.gc([solution]);
    //let solution = renamer.rename(solution).unwrap();
    let solutions : G = factory.number_solutions(solution);
    //let gc_len = factory.len();
    //println!("Original len {} gc len {} solutions {}",original_len,gc_len,solutions);
    solutions
}



fn count_dominoes_bdd() {
    let solutions = count_tiling::<BDDFactory<u32,NoMultiplicity>,u64>(setup_chessboard_tiled_with_dominoes(8));
    assert_eq!(solutions,12988816); // See Knuth, "The art of Computer programming Volume 4, Fascicle 1, Binary Decision Diagrams", section 7.1.4, p119
}


fn count_dominoes_zdd() {
    let solutions = count_tiling::<ZDDFactory<u32,NoMultiplicity>,u64>(setup_chessboard_tiled_with_dominoes(8));
    assert_eq!(solutions,12988816); // See Knuth, "The art of Computer programming Volume 4, Fascicle 1, Binary Decision Diagrams", section 7.1.4, p119
}



fn count_dominoes_dynamic_programming() {
    let mut input_buffer = vec![0u64;256];
    let mut output_buffer = vec![0u64;256];
    input_buffer[0]=1;
    for _y in 0..8 {
        for x in 0..8 {
            let mask : usize = 1<<x;
            for i in 0..256 {
                let already_occupied = (i&mask)!=0;
                let count = input_buffer[i];
                if already_occupied {
                    output_buffer[i-mask]+=count;
                } else {
                    // domino downwards
                    output_buffer[i+mask]+=count;
                    // domino right
                    if x<7 && i&(mask<<1) == 0 { output_buffer[i+(mask<<1)]+=count}
                }
                input_buffer[i]=0;
            }
            mem::swap(&mut input_buffer,&mut output_buffer);
        }
    }
    let solutions = input_buffer[0];
    assert_eq!(solutions,12988816); // See Knuth, "The art of Computer programming Volume 4, Fascicle 1, Binary Decision Diagrams", section 7.1.4, p119
}



fn count_up_to_trionimoes_bdd() {
    let solutions = count_tiling::<BDDFactory<u32,NoMultiplicity>,u128>(setup_chessboard_tiled_with_up_to_trionimoes(8));
    assert_eq!(solutions,92109458286284989468604); // See Knuth, "The art of Computer programming Volume 4, Fascicle 1, Binary Decision Diagrams", section 7.1.4, p120
}


fn count_up_to_trionimoes_zdd() {
    let solutions = count_tiling::<ZDDFactory<u32,NoMultiplicity>,u128>(setup_chessboard_tiled_with_up_to_trionimoes(8));
    assert_eq!(solutions,92109458286284989468604); // See Knuth, "The art of Computer programming Volume 4, Fascicle 1, Binary Decision Diagrams", section 7.1.4, p120
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count dominoes BDD", |b| b.iter(|| count_dominoes_bdd()));
    c.bench_function("count dominoes ZDD", |b| b.iter(|| count_dominoes_zdd()));
    c.bench_function("count dominoes DP", |b| b.iter(|| count_dominoes_dynamic_programming()));
    let mut slower = c.benchmark_group("count up to trionimoes");
    slower.sample_size(10);
    slower.bench_function("count up to trionomoes BDD", |b| b.iter(|| count_up_to_trionimoes_bdd()));
    slower.bench_function("count up to trionomoes ZDD", |b| b.iter(|| count_up_to_trionimoes_zdd()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
