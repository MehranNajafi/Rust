use chrono;
use chrono::Datelike; // 0.4.19
use std::cmp::Ordering;
fn main() {
    //     println!("Hello\n world!");
    //     let id = 12;
    //     println!("{}",id);
    //     let mut a = 24;
    //     a = 100;
    //     println!("{}",a);
    //     let b;
    //     b= 150;
    //     println!("{} {}",b,a);
    //     let str = "abcd";
    //     println!("{} {}",str::len("abc"),str.len());
    // ifchange();
    // whileLoop();
    // for_Loop();
    // array();
    // rust_attr();
    // vec_use();
    // play_with_arr();
    // arr_play();
    // enum_with_type();
    // struct_sample();
    // print!("{}",make_it_double(24));
    // print!("{}",final_Price(4, 400.0));
    // let result = tuple_as_return("Mehran", "Najafi", 30);
    // print!("{}, birth year is : {}", result.0, result.1);
    // let result = generic('a', 1, 2);
    // print!("{}", result);
    // struct_generic();
    //  genericFile::use_swap();
    // genericFile::arr_pop();
    // divide_file::show_divide(12.,2.);
    // size();
    // calc_size();
    // calc_size_with_capacity();
    sort_arr();
}
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
