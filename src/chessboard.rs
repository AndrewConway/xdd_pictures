use crate::tiling_problem::TilingProblem;

/// Define a tiling problem as a chessboard with dominoes.
pub fn setup_chessboard_tiled_with_dominoes(side_length_wanted:usize) -> TilingProblem<[i32;2]> {
    let mut problem = TilingProblem::default();
    for y in 0..side_length_wanted as i32 {
        for x in 0..side_length_wanted as i32 {
            problem.add_site([x,y]);
        }
    }
    for y in 0..side_length_wanted as i32 {
        for x in 0..side_length_wanted as i32 {
            // add tile going to right
            problem.add_tile_containing_sites(&[[x,y],[x+1,y]]);
            // add tile going down.
            problem.add_tile_containing_sites(&[[x,y],[x,y+1]]);
        }
    }
    assert_eq!(side_length_wanted*(side_length_wanted-1)*2,problem.tiles.len());
    problem
}

/// Define a tiling problem as a chessboard with mononimoes, dominoes, and trionimoes.
pub fn setup_chessboard_tiled_with_up_to_trionimoes(side_length_wanted:usize) -> TilingProblem<[i32;2]> {
    let mut problem = TilingProblem::default();
    for y in 0..side_length_wanted as i32 {
        for x in 0..side_length_wanted as i32 {
            problem.add_site([x,y]);
        }
    }
    for y in 0..side_length_wanted as i32 {
        for x in 0..side_length_wanted as i32 {
            // add mononimo
            problem.add_tile_containing_sites(&[[x,y]]);
            // add tile going to right
            problem.add_tile_containing_sites(&[[x,y],[x+1,y]]);
            // add tile going down.
            problem.add_tile_containing_sites(&[[x,y],[x,y+1]]);
            // add trionimoes
            problem.add_tile_containing_sites(&[[x,y],[x+1,y],[x+2,y]]);
            problem.add_tile_containing_sites(&[[x,y],[x+1,y],[x+1,y+1]]);
            problem.add_tile_containing_sites(&[[x,y],[x+1,y],[x,y+1]]);
            problem.add_tile_containing_sites(&[[x,y],[x+1,y+1],[x,y+1]]);
            problem.add_tile_containing_sites(&[[x,y],[x,y+1],[x,y+2]]);
            problem.add_tile_containing_sites(&[[x+1,y],[x,y+1],[x+1,y+1]]);
        }
    }
    problem
}