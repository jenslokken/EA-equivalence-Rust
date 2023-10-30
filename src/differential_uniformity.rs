
fn calculate_diff(f: &Vec<u32>, size: u32) {
    let dim = 1<<size;
    let f = f[1..].to_vec();
    let mut max:u32 = 0;
    let mut hit = [0;1<<14];
    for a in 1..(dim) {
        for x in 0..(dim) {
            let value = f[a^x]^f[x];
            hit[value as usize] += 1;
        }
        for i in 0..(dim) {
            if hit[i] > max {
                max = hit[i];
            }
            hit[i] = 0;
        }
    }
    println!("The differential uniformity of f is: {}", max);
}
