use std::collections::HashSet;
use std::collections::HashMap;

pub fn bucket_partition_by_image(truth_table: &Vec<u32>) -> (Vec<u32>, Vec<HashSet<usize>>) {
    let length:usize = truth_table.len();
    let mut buckets:Vec<HashSet<usize>> = Vec::new();
    let mut indices:Vec<u32> = vec![0; length];
    let mut count_map: HashMap<u32, u32> = HashMap::new();

    for i in 0..length {
        let output = truth_table[i];
        buckets.insert(0, HashSet::new());
        if count_map.contains_key(&output) {
            let count = count_map.get(&output).unwrap();
            count_map.insert(output, count + 1);
        } else {
            count_map.insert(output, 1);
        }
    }
    for i in 0..length {
        let count = match count_map.get(&(i as u32)) {
            None => 0,
            Some(x) => *x,

        };
        indices[i] = count;
        buckets[count as usize].insert(i);
    }
    return (indices, buckets);
}

pub fn bucket_partition_by_quadruple(truth_table: &Vec<u32>) -> (Vec<u32>, Vec<HashSet<usize>>) {
    let size:usize = truth_table.len();
    let mut buckets:Vec<HashSet<usize>> = Vec::new();
    let mut indices:Vec<u32> = vec![0; size];
    let mut count_map: HashMap<u32, u32> = HashMap::new();
    for i in 0..size {
        for j in i..size {
            for k in j..size {
                let quadruple = truth_table[i] ^ truth_table[j] ^ truth_table[k] ^ truth_table[i ^ j ^ k];
                let count = count_map.entry(quadruple).or_insert(0);
                *count += 1;
            }
        }
    }
    //collect the map into a vector and sort it by the count
    let mut sorted_count: Vec<(&u32, &u32)> = count_map.iter().collect();
    sorted_count.sort_by(|a, b| a.1.cmp(b.1)); 
    let mut current = sorted_count.get(0).unwrap().1;

    //create a bucket for each count
    let mut bucket: HashSet<usize> = HashSet::new();
    for i in 0..size {
        let (value, count) = sorted_count.get(i).unwrap();
        if *count == current {
            bucket.insert(**value as usize);
        } else {
            buckets.push(bucket);
            bucket = HashSet::new();
            bucket.insert(**value as usize);
            current = count;
        }
        indices[**value as usize] = buckets.len() as u32;
    }
    //add the last bucket
    buckets.push(bucket);
    return (indices, buckets);
}

fn walsh_transform_helper(a:u32, b:u32, f_x:u32, x:u32) -> u32{
    let calc_1 = a&x;
    let calc_2 = b&f_x;
    
    return calc_1.count_ones()^calc_2.count_ones();
}

pub fn walsh_transform(a:u32, b:u32, truth_table: &Vec<u32>) -> i32 {
    let mut result:i32 = 0;
    let length:usize = truth_table.len();
    for x in 0..length {
        let f_x = truth_table[x];
        result += (-1 as i32).pow(walsh_transform_helper(a, b, f_x, x as u32));
    }
    return result;
}
