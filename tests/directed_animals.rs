use xdd::{BDDFactory, DecisionDiagramFactory, NoMultiplicity, ZDDFactory};
use chessboard_tiling_pictures::directed_animals::{count_directed_animals_by_memoization, count_directed_animals_xdd};

#[test]
fn test_memoization() {
    let (cache_size,values) = count_directed_animals_by_memoization(20);
    println!("Memoization cache size {cache_size}");
    assert_eq!(vec![1,1,2,5,13,35,96,267,750,2123,6046,17303,49721,143365,414584,1201917,3492117,10165779,29643870,86574831], values);
}

fn test_xdd<F: DecisionDiagramFactory<u32,NoMultiplicity>>(name:&str) {
    let (original_size,factory,_node,values) = count_directed_animals_xdd::<F>(20);
    assert_eq!(vec![1,1,2,5,13,35,96,267,750,2123,6046,17303,49721,143365,414584,1201917,3492117,10165779,29643870,86574831], values);
    println!("{name} Used {original_size} nodes ({} after GC)",factory.len());
}


#[test]
fn count_bdd() {
    test_xdd::<BDDFactory<u32,NoMultiplicity>>("BDD")
}

#[test]
fn count_zdd() {
    test_xdd::<ZDDFactory<u32,NoMultiplicity>>("ZDD")
}
