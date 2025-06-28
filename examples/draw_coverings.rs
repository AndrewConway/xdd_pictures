use lattice_picture::geometry::{Cell, SquareLattice};
use lattice_picture::item::LatticeItem;
use lattice_picture::picture::LatticePicture;
use lattice_picture::svg::{SizedSVG};
use xdd::{BDDFactory, DecisionDiagramFactory, NoMultiplicity, VariableIndex};
use xdd_pictures::chessboard::{setup_chessboard_tiled_with_dominoes, setup_chessboard_tiled_with_up_to_trionimoes};
use xdd_pictures::directed_animals::{count_directed_animals_xdd, variable_number};
use xdd_pictures::tiling_problem::{SiteIndex, TilingProblem};

fn count_tiling<F: DecisionDiagramFactory<u32, NoMultiplicity>>(problem:TilingProblem<[i32;2]>,to_draw:&[u128],name:&str)  {
    let (mut factory ,solution) = problem.find_tiling_solution::<F>();
    let original_len = factory.len();
    let renamer = factory.gc([solution]);
    let solution = renamer.rename(solution).unwrap();
    let finder = factory.find_all_solutions_with_minimal_true_arguments::<u128>(solution,problem.tiles.len() as u16);
    let gc_len = factory.len();
    println!("Original len {} gc len {} solutions {}",original_len,gc_len,finder.number_solutions());
    for i in to_draw {
        let sol = finder.get_ith_solution(*i).unwrap();
        let pic = draw_solution(&problem, &sol);
        std::fs::write(format!("{name}{}.svg",*i),format!("{}",pic.svg)).unwrap();
        
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

fn draw_directed_animal(terms_wanted:u16) {
    let  (_original_size,factory,node,_result) = count_directed_animals_xdd::<BDDFactory<u32,NoMultiplicity>>(terms_wanted);
    let generator = factory.find_all_solutions::<u128>(node,variable_number(0,terms_wanted).0);
    for i in 1000000..1500000 {
        if let Some(solution) = generator.get_ith_solution(i) {
            println!("DirAn {i} has {} elements",solution.len());
            if solution.len()==terms_wanted as usize {
                let mut picture = LatticePicture::default();
                for x in 0..terms_wanted {
                    for y in 0..terms_wanted {
                        if solution.contains(&variable_number(x,y)) {
                            picture.items.push(LatticeItem::Polyonimo(vec![Cell{x:x as i32,y:-(y as i32)}]))
                        }
                    }
                }
                let svg = picture.to_svg::<SquareLattice>(10.0);
                std::fs::write(format!("DirectedAnimal{i}.svg"),format!("{}",svg.svg)).unwrap();
            }
        }
    }
}


fn main() {
    count_tiling::<BDDFactory<u32,NoMultiplicity>>(setup_chessboard_tiled_with_dominoes(8),&[0,1,5134723,5134724],"dominoes"); // 0,1,5134723,5134724
    count_tiling::<BDDFactory<u32,NoMultiplicity>>(setup_chessboard_tiled_with_up_to_trionimoes(8),&[0,1,173057372831,243057372831],"trionimoes");
    draw_directed_animal(10);
}