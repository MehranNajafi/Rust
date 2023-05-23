fn range_sample() {
    let range: std::ops::Range<usize> = 5..15;
    print!(
        "{:?} {} {} {} \n",
        range,
        range.start,
        range.end,
        range.len()
    );
    for i in range {
        print!("{} .", i);
    }
}
fn size_of_var() {
    let v1 = 2u8..12u8;
    let v2 = 2..12u8;
    let v3 = 2u8..12;
    let v4 = 2..12;
    let v5 = -2..12;
    let v6 = 2..12 as i64;
    print!(
        "{} {} {} {} {} {} ",
        std::mem::size_of_val(&v1),
        std::mem::size_of_val(&v2),
        std::mem::size_of_val(&v3),
        std::mem::size_of_val(&v4),
        std::mem::size_of_val(&v5),
        std::mem::size_of_val(&v6)
    )
}
fn min(arr: &[i32]) -> i32 {
    // Let's assume 'arr' is not empty.
    let mut minimum = arr[0];
    for i in 1..arr.len() {
        if arr[i] < minimum {
            minimum = arr[i];
        }
    }
    minimum
}
fn arr_slice() {
    let arr = [55, 22, 33, 44, 66, 7, 8];
    let v = vec![55, 22, 33, 44, 66, 7, 8];
    let sr1 = &arr[2..5];
    let sr2 = &v[2..5];
    print!("{:?} {:?} {:?} {:?}", sr1, sr2, &sr1[1..2], &sr1[1]);
}
fn mutable_slice() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    let slice1 = &mut arr[2..5];
    print!("{:?}", slice1);
    slice1[2] = 0;
    print!("{:?}", arr);
}
fn open_ended_slice() {
    let r1: std::ops::RangeFrom<i32> = 3..;
    let r2: std::ops::RangeTo<i32> = ..15;
    print!(
        "{:?} {:?} {} {}",
        r1,
        r2,
        std::mem::size_of_val(&r1),
        std::mem::size_of_val(&r2),
    );
}
fn range_inclusive() {
    let arr = [2, 4, 6, 8, 10];
    let r1: std::ops::RangeInclusive<usize> = 2..=3;
    print!("{:?}", &arr[r1]);
    let r2: std::ops::RangeToInclusive<usize> = ..=2;
    print!("{:?}", &arr[r2]);
}
