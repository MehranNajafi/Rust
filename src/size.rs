fn size() {
    println!("{}", std::mem::size_of::<i32>());
    println!("{}", std::mem::size_of_val(&12));
}
fn calc_size() {
    let mut val = vec![0; 0];
    let mut prev_capacity = std::usize::MAX;
    for i in 0..1000 {
        let cap = val.capacity();
        if cap != prev_capacity {
            println!("{} {} {}", i, val.len(), cap);
            prev_capacity = cap;
        }
        val.push(1);
    }
}
fn calc_size_with_capacity() {
    let mut val = Vec::with_capacity(200);
    let mut prev_capacity = std::usize::MAX;
    for i in 0..1000 {
        let cap = val.capacity();
        if cap != prev_capacity {
            println!("{} {} {}", i, val.len(), cap);
            prev_capacity = cap;
        }
        val.push(1);
    }
}