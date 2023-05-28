fn iterator_string(s: &str) {
    let mut itr_str = s.chars();
    loop {
        match itr_str.next() {
            Some(c) => {
                println!("{} : {}", c, c as u32);
            }
            None => {
                break;
            }
        }
    }
}
fn vec_iterator() {
    for item in vec![22, 33, 44].into_iter() {
        let j = item;
        print!("{}", j + 1);
    }
}
fn itr_gen_refrence() {
    let arr: std::slice::Iter<i32> = [1, 2, 3, 4, 5, 6][2..5].iter();
    for item in arr {
        print!("{} -", *item + 1);
    }
}
fn use_iter_filter() {
    //avoid to write condition statement
    let arr = [0, 20, -30, 50, -12, 44, 23, -19];
    for n_arr in arr.into_iter().filter(|x| *x < 0) {
        print!("{} - ", n_arr);
    }
}
fn use_iter_map() {
    //avoid to write * 2 for each array prop
    let arr = [2, 4, 6, 8, 10, 12, 14, 16];
    for d_arr in arr.into_iter().map(|x| x * 2) {
        print!("{} - ", d_arr);
    }
}
fn iter_usage() {
    //avoid to use len func to get array count
    let arr = [2, 4, 6, 8, 10, 12, 14, 16];
    for p_arr in arr.into_iter() {
        print!("{} - ", p_arr);
    }
    //even if you need the index of array you can use this
    for (index, v_arr) in arr.into_iter().enumerate() {
        print!("{} {} -", index, v_arr);
    }
}
fn iter_any_all() {
    let arr = [2, 4, -6, 8, -10, -12, 14, -16];
    print!("{} ", arr.into_iter().any(|x| -> bool { x < 0 }));
    //if you want to check all of array items
    print!("{} ", arr.into_iter().all(|x| -> bool { x < 0 }));
}
fn iter_count_size() {
    let text = "mehran";
    print!("{} {} ", text.chars().count(), text.len());
}
fn iter_sum_min_max() {
    let arr = [2, 4, -6, 8, -10, -12, 14, -16];
    print!("{} \n", arr.into_iter().sum::<i32>());
    match arr.into_iter().min() {
        Some(n) => print!("{} \n", n),
        _ => (),
    }
    match arr.into_iter().max() {
        Some(n) => print!("{} \n", n),
        _ => (),
    }
}
fn iter_collect() {
    let arr = [2, 4, -6, 8, -10, -12, 14, -16];
    let v_arr = arr.into_iter().collect::<Vec<_>>();
    let text = "mehran";
    print!("{} ", text.chars().collect::<String>());
    print!("{:?} ", text.chars().collect::<Vec<char>>());
}
fn iter_chain() {
    let arr = [2, 4, -6, 8, -10, -12, 14, -16];
    let r = arr
        .into_iter()
        .filter(|x| *x < 0)
        .map(|x| x + 2)
        .collect::<Vec<_>>();
    print!("{:?}", r);
}
fn iter_lazy() {
    let arr = [2, 4, -6, 8, -10, -12, 14, -16];
    let result = arr
        .into_iter()
        .filter(|x| {
            print!("N{} ", x);
            *x < 0
        })
        .map(|x| {
            print!("Add{} ", x);
            x + 2
        })
        .collect::<Vec<_>>();
    print!("{:?}", result);
}
