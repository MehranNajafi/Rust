fn desc(a: &i32, b: &i32) -> Ordering {
    if (a < b) {
        Ordering::Greater
    } else if (a > b) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
fn asc(a: &i32, b: &i32) -> Ordering {
    if (a > b) {
        Ordering::Greater
    } else if (a < b) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
fn sort_arr() {
    let mut arr = [5, 8, 3, 6, 2, 9, 5, 2, 0];
    let asc_sort = |a: &i32, b: &i32| -> Ordering {
        if (a > b) {
            Ordering::Greater
        } else if (a < b) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };
    // arr.sort_by(asc_sort);
    arr.sort_by(|a,b| b.cmp(a));
    print!("{:?}", arr);
}
