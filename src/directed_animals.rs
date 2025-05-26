//! Count directed animals, comparing the XDD approach to a traditional recursive memoization approach.
//!
//! Each variable in the XDD corresponds to the presence or absence of a given site.
//! To get up to length n, need the triangular lattice with coordinates (x,y) such that x>=0, y>=0, and x+y<n
//! We need to number sites. Do a triangular counting. Diagonal d=x+y has numbers starting from d*(d+1)/2.
//! Site (x,y) has number x+d*(d+1)/2, where d=x+y.
//!
//! The site constraint means that for each site (x,y) other than (0,0),
//! at least one of the prior sites (x-1,y) and (x,y-1) must be present as long as it is present.
//! That is, the function we need to compute is the intersection (logical and) of
//! one term for each site other than the origin being (x-1,y) | (x,y-1) | !(x,y)


use std::collections::HashMap;
use xdd::{DecisionDiagramFactory, NodeIndex, NoMultiplicity, VariableIndex};
use xdd::generating_function::{SingleVariableGeneratingFunctionFixedLength};


pub fn count_directed_animals_by_memoization(length:u32) -> (usize,Vec<u64>) {
    fn count_work(cache:&mut HashMap<(u64,u64,u32),u64>,sig_this_row:u64,sig_next_row:u64,n:u32) -> u64 {
        if n==0 { 1 }
        else if sig_this_row==0 {
            if sig_next_row == 0 { 0 } else { count_work(cache, sig_next_row, 0, n) }
        } else {
            let key = (sig_this_row,sig_next_row,n);
            cache.get(&key).cloned().unwrap_or_else(||{
                let next_choice = sig_this_row&(1+!sig_this_row); // single bit, can't be zero.
                let removed_choice = sig_this_row& !next_choice; // removed but.
                let choice0 = count_work(cache,removed_choice,sig_next_row,n);
                let choice1 = count_work(cache,removed_choice,sig_next_row|(3*next_choice),n-1);
                let res = choice0+choice1;
                cache.insert(key,res);
                res
            })
        }
    }
    let mut cache = HashMap::new();
    let mut res = vec![];
    for i in 0..length {
        res.push(count_work(&mut cache,1,0,i));
    }
    (cache.len(),res)
}



fn variable_number(x:u16,y:u16) -> VariableIndex {
    let d = x+y;
    VariableIndex(x+(d*(d+1))/2)
}

/// Count using a decision diagram
pub fn count_directed_animals_xdd<F: DecisionDiagramFactory<u32,NoMultiplicity>>(terms_wanted:u16) -> (usize,F, NodeIndex<u32, NoMultiplicity>, Vec<u64>) {
    let num_variables = variable_number(0,terms_wanted).0;
    let mut factory = F::new(num_variables);
    let mut function : Option<NodeIndex<u32,NoMultiplicity>> = None;
    for x in 0..terms_wanted {
        for y in 0..(terms_wanted-x) {
            // println!("Working on node ({},{})",x,y);
            // std::io::stdout().flush().unwrap();
            if x>0 || y>0 {
                let variable_here = factory.single_variable(variable_number(x,y));
                let not_variable_here = factory.not(variable_here);
                let left = if x>0 { factory.single_variable(variable_number(x-1,y)) } else { NodeIndex::FALSE };
                let below = if y>0 { factory.single_variable(variable_number(x,y-1)) } else { NodeIndex::FALSE };
                let prior = factory.or(left,below);
                let term = factory.or(prior,not_variable_here);
                function = Some(if let Some(f) = function {factory.and(term,f)} else {term});
            }
        }
    }
    let original_size = factory.len();
    let map = factory.gc([function.unwrap()]);
    let node = map.rename(function.unwrap()).unwrap();
    //factory.print(function.unwrap());
    let result = factory.number_solutions::<SingleVariableGeneratingFunctionFixedLength<20>>(node);
    let result = result.0;
    (original_size,factory,node,result)
}
