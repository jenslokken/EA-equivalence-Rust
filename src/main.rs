use std::env;

use bucket_partition::walsh_transform;
mod parse;
mod outer_permutation;
mod bucket_partition;
mod inner_permutation;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path_f = args.get(1);
    let file_path_g = args.get(2);
    if file_path_f == None || file_path_g == None {
        println!("Error: missing arguments");
        return;
    }
       
    let (_s, f) = parse::parse_input(file_path_f.expect("Expected 1 argument"));
    let g = file_path_g.unwrap();
    if g == "-w" {
        let a = args.get(3);
        let b = args.get(4);
        let a = a.expect("Expected 2 arguments");
        let b = b.expect("Expected 2 arguments");
        let walsh_result = walsh_transform(a.parse().unwrap(), b.parse().unwrap(), &f);
        print!("{}", walsh_result);

    } else {
        let (indices_f, buckets_f) = bucket_partition::bucket_partition_by_quadruple(&f);
        let (size, g) = parse::parse_input(file_path_g.expect("Expected 2 arguments"));
        let (indices_g, buckets_g) = bucket_partition::bucket_partition_by_quadruple(&g);
        let mut guessed: Vec<u32> = vec![0; size as usize];
        let mut field: Vec<u32> = vec![0; 1 << size]; // truth table initialized with 2^n zeros 
        print!("buckets_f: {:?}\n", buckets_f);
        print!("buckets_g: {:?}\n", buckets_g);
        
        print!("indices_f: {:?}\n", indices_f);
        print!("indices_g: {:?}\n", indices_g);
        if outer_permutation::validate_bucket_partition(&indices_f, &indices_g) {
            print!("buckets are valid\n");
            outer_permutation::search(&indices_f, &indices_g, &buckets_f, &buckets_g,&mut guessed, 0, size, &mut field, &f, &g);
        } else {
            print!("buckets are invalid\n");
        }
    }

}
