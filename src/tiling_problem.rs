/// Copyright 2022-2025 Andrew Conway. All rights reserved. See README.md for licensing. 

use std::collections::HashMap;
use std::hash::Hash;
use xdd::{DecisionDiagramFactory, NoMultiplicity, NodeIndex, VariableIndex};

pub type SiteIndex = usize;
pub type Tile = Vec<SiteIndex>;
pub type TileIndex = usize;

#[derive(Default)]
/// A worker to make it straight forward to set up a tiling problem:
/// * Add all sites with add_site
/// * Add all times with add_tile or add_tile_containing_sites
/// * call find_tiling_solution to get an xDD whose solutions have each site covered by exactly one tile.
pub struct TilingProblem<Site> {
    pub sites : Vec<Site>,
    site_index_by_site : HashMap<Site,SiteIndex>,
    pub tiles : Vec<Tile>,
    /// tiles_covering_a_site[site_index] is a list containing tile_index iff tiles[tile_index] contains site_index.
    tiles_covering_a_site : Vec<Vec<TileIndex>>,
}

impl <Site: Clone+Hash+Eq> TilingProblem<Site> {
    pub(crate) fn add_site(&mut self, s:Site) -> SiteIndex {
        let index = self.sites.len();
        self.sites.push(s.clone());
        self.site_index_by_site.insert(s,index);
        self.tiles_covering_a_site.push(Vec::new());
        index
    }
    fn add_tile(&mut self,tile:Tile) {
        let index = self.tiles.len();
        for &s in &tile {
            self.tiles_covering_a_site[s].push(index);
            self.tiles_covering_a_site[s].sort();
        }
        self.tiles.push(tile);
    }
    /// If all the sites on the tile exist, add it and return true. Otherwise return false.
    pub(crate) fn add_tile_containing_sites(&mut self, sites:&[Site]) -> bool {
        let mut tile = Vec::new();
        for s in sites {
            if let Some(index) = self.site_index_by_site.get(s) { tile.push(*index); } else { return false; }
        }
        self.add_tile(tile);
        true
    }
    pub fn find_tiling_solution<F: DecisionDiagramFactory<u32,NoMultiplicity>>(&self) -> (F, NodeIndex<u32,NoMultiplicity>) {
        let mut factory = F::new(self.tiles.len() as u16);
        let mut constraints = Vec::new();
        for tiles_covering_site in &self.tiles_covering_a_site {
            let constraint_for_that_site = factory.exactly_one_of(& tiles_covering_site.iter().map(|t|VariableIndex(*t as u16)).collect::<Vec<_>>());
            constraints.push(constraint_for_that_site);
        }
        constraints.reverse(); // much faster to merge later tiles first.
        let node = factory.poly_and(&constraints).unwrap();
        (factory,node)
    }
}
