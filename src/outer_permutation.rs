use std::collections::HashSet;
use std::collections::HashMap;
use crate::inner_permutation::compute_inner_permutation;
use crate::utils;
use rayon::prelude::*;

fn count_bucket_size(indices: &Vec<u32>) -> HashMap<u32, u32> {
    let mut count_map: HashMap<u32, u32> = HashMap::new();
    for i in indices{
        let count = count_map.entry(*i).or_insert(0);
        *count += 1;
    }
    return count_map;
}

pub fn validate_bucket_partition(indices_f:&Vec<u32>, indices_g:&Vec<u32>) -> bool {
    let count_map_f = count_bucket_size(indices_f);
    let count_map_g = count_bucket_size(indices_g);
    for i in count_map_f.keys() {
        if count_map_f.get(i) != count_map_g.get(i) {
            return false;
        }
    }
    return true;
    

}

fn validate(
    bucket_indices_f:&Vec<u32>,
    bucket_indices_g:&Vec<u32>,
    basis: u32,
    basis_images:&Vec<u32>,
    truth_table: &mut Vec<u32>,


) -> bool {
    for i in 0..(1 << basis) {
        let x:usize = i^(1<< basis);
        let y = truth_table[i]^basis_images[basis as usize];
        
        if bucket_indices_f[x] != bucket_indices_g[y as usize] {
            return false;
        }
        truth_table[x] = y;
    }
    return true;

}

fn output (_guessed: &Vec<u32>, dimension: u32, f: &Vec<u32>, g: &Vec<u32>, truth_table: &Vec<u32>) {
    let l_inverse = utils::inverse(truth_table);
    let g = &utils::compose(&l_inverse, g);
    compute_inner_permutation( dimension, f, g, &l_inverse, truth_table);
    
    
}

pub fn search(
    indices_f: &Vec<u32>, 
    indices_g: &Vec<u32>, 
    buckets_f: &Vec<HashSet<usize>>, 
    buckets_g: &Vec<HashSet<usize>>, 
    guessed: &mut Vec<u32>, 
    basis_index: u32,
    dimension: u32,
    truth_table: &mut Vec<u32>,
    f: &Vec<u32>,
    g: &Vec<u32>,
) {
    if basis_index == dimension {
        output(guessed, dimension, f, g,  &truth_table);
        return;
    }

    let bucket_index = indices_f[1<<(basis_index)] as usize;
    let elements = buckets_g[bucket_index].par_iter();
    
    elements.for_each(|elem|
        {
            let mut fresh_guessed = guessed.clone();
            fresh_guessed[basis_index as usize] = *elem as u32;
            let mut new_truth_table = truth_table.clone();
            
            if validate(indices_f, indices_g, basis_index, &fresh_guessed, &mut new_truth_table) {
                search(indices_f, indices_g, buckets_f, buckets_g, &mut fresh_guessed, basis_index + 1, dimension, &mut new_truth_table, f, g);
            }
        }

    )


}
