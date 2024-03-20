fn main() {
    let prime = 23291_usize;
    for l in 0..100 {
        let mut v = vec![0; l];

        for i in 0..l {
            v[(l/2 + prime*i) % l] = i + 1;
        }
        let ok = v.iter().all(|x| *x > 0);
        println!("{}: {:?}", ok, v);
    }
}