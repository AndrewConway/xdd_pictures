use lattice_picture::geometry::{Cell, SquareLattice};
use lattice_picture::item::LatticeItem;
use lattice_picture::picture::LatticePicture;
use lattice_picture::svg::{SizedSVG};
use xdd::{BDDFactory, DecisionDiagramFactory, NoMultiplicity, VariableIndex};
use chessboard_tiling_pictures::chessboard::setup_chessboard_tiled_with_dominoes;
use chessboard_tiling_pictures::tiling_problem::{SiteIndex, TilingProblem};

fn count_tiling<F: DecisionDiagramFactory<u32, NoMultiplicity>>(problem:TilingProblem<[i32;2]>,to_draw:&[u128])  {
    let (mut factory ,solution) = problem.find_tiling_solution::<F>();
    let original_len = factory.len();
    let renamer = factory.gc([solution]);
    let solution = renamer.rename(solution).unwrap();
    let finder = factory.find_all_solutions::<u128>(solution,problem.tiles.len() as u16);
    let gc_len = factory.len();
    println!("Original len {} gc len {} solutions {}",original_len,gc_len,finder.number_solutions());
    for i in to_draw {
        let sol = finder.get_ith_solution(*i).unwrap();
        let pic = draw_solution(&problem, &sol);
        std::fs::write(format!("dominoes{}.svg",*i),format!("{}",pic.svg)).unwrap();
        
    }
}


fn draw_solution(problem:&TilingProblem<[i32;2]>,solution:&[VariableIndex]) -> SizedSVG {
    let mut picture = LatticePicture::default();
    let siteindex_to_cell = |si: &SiteIndex| -> Cell {
        let site = problem.sites[*si];
        Cell{x:site[0],y:site[1]}
    };
    for v in solution {
        let tile = &problem.tiles[v.0 as usize];
        picture.items.push(LatticeItem::Polyonimo(tile.iter().map(siteindex_to_cell).collect()));
    }
    let svg = picture.to_svg::<SquareLattice>(10.0);
    svg
}




fn main() {
    count_tiling::<BDDFactory<u32,NoMultiplicity>>(setup_chessboard_tiled_with_dominoes(8),&[0,1,5134723,5134724]);
}