use std::{collections::HashSet};

use crate::utils::{self, compose};

pub fn compute_domains(dimension: u32, f: &Vec<u32>, g: &Vec<u32>) -> Vec<HashSet<u32>> {
    //Function for computing  the domains of G from the standard basises.
    let mut domains: Vec<HashSet<u32>> = Vec::new();
    let size:usize = 1 << dimension;
    let mut O:Vec<HashSet<u32>> = vec![HashSet::new(); size]; 
    for x_1 in 0..size {
        domains.push(HashSet::from_iter(0..(size as u32))); //initialize domains with 2^n elements
        for x_2 in 0..size {
            let t = f[x_1] ^ f[x_2] ^ f[x_1^x_2];
            O[t as usize].insert(x_1 as u32);
            O[t as usize].insert(x_2 as u32);
            O[t as usize].insert((x_1 ^ x_2) as u32);
        }
    }
    for x_1 in 0..size {
        for x_2 in 0..size {
            let t = g[x_1] ^ g[x_2] ^ g[x_1 ^ x_2];
            let intersection_x1 = &domains[x_1] & &O[t as usize];
            let intersection_x2 = &domains[x_2] & &O[t as usize];
            let intersection_x1_x2 = &domains[x_1 ^ x_2] & &O[t as usize];
            domains[x_1] = intersection_x1;
            domains[x_2] = intersection_x2;
            domains[x_1 ^ x_2] = intersection_x1_x2;
        }
    }

    return domains;
}

pub fn compute_inner_permutation(dimension: u32, f: &Vec<u32>, g:&Vec<u32>, l_inverse: &Vec<u32>, l_1: &Vec<u32>) {
    let domains: Vec<HashSet<u32>> = compute_domains(dimension, f, g);
    let mut domains_basis: Vec<Vec<u32>> = Vec::new(); // domains of the basiselements
    let mut computed_basises: Vec<u32> = vec![0;dimension as usize];
    for b in 0..dimension {
        let mut temp:Vec<u32> = Vec::new();
        let dom = domains.get(1 << b).unwrap();
        for x in dom {
            temp.push(*x);
        }
        domains_basis.push(temp);
    }
    //println!("domains {:?}", domains_basis);
    dfs( 0, dimension, f, g, l_inverse, l_1, &mut computed_basises, &domains_basis);
    
}

fn output(l_1: &Vec<u32>, l_2: Vec<u32>, a: Vec<u32>) {
    println!("L1: {:?}", l_1);
    println!("L2: {:?}", l_2);
    println!("A: {:?}", a);
    println!("\n");
}

fn dfs(
    basis_index: u32, 
    dimension: u32, 
    f: &Vec<u32>, 
    g: &Vec<u32>, 
    l_inverse: &Vec<u32>,
    l_1: &Vec<u32>,
    computed_basises:&mut Vec<u32>, 
    domains:&Vec<Vec<u32>>, 
) {
    let domain = &domains[basis_index as usize];
    for d in domain {        
        computed_basises[basis_index as usize] = *d;
        
        if basis_index == (dimension - 1) {
            // println!("computed basis {:?}", computed_basises);
            // L1 * f * L2 + A = G
            // f * L2 + A' = G'
            // A' = f*L2 + G'
            let l_2 = utils::create_tt(&computed_basises);
            let l_f = utils::compose(f, &l_2); 
            let a_prime = utils::sum(&l_f, g);
            let a = utils::compose(l_1, &a_prime);
            
            if !utils::is_affine(&a_prime, dimension) {
                continue;
            }
            if !utils::is_bijective(&l_2) {
                continue;
            }
            output(l_1, l_2, a);
            
        } else {
            dfs( basis_index + 1, dimension, f, g, l_inverse, l_1, computed_basises, domains);
        }
    }
}
