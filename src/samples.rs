
fn old_main(){
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
    // sort_arr();
    // string_type();
    // string_as_pointer();
}
fn struct_generic(){
    #[allow(dead_code)]
    struct gen_struct<T1,T2>{
        ch:char,
        n:T1,
        n1:T2
    }
    let var = gen_struct::<i32,f32>{ch:'c',n:2,n1:2.1};
    print!("{}",var.n1);
}
fn generic<T>(ch: char, num1: T, num2: T) -> T {
    if (ch == 'a') {
        return num1;
    } else {
        return num2;
    }
}
fn tuple_as_return(fName: &str, lName: &str, age: i32) -> (String, i32) {
    let current_date = chrono::Utc::now();
    let year: i32 = current_date.year();
    let birthYear: i32 = year - age;
    let mut full_name = String::from(fName.to_owned());
    full_name.push_str(" ");
    full_name.push_str(lName);
    return (full_name, birthYear);
}
fn final_Price(count: i32, price: f64) -> f64 {
    if (price <= 0. || count <= 0) {
        return 0.;
    }
    return count as f64 * price;
}
fn make_it_double(x: i128) -> i128 {
    x * 2
}
fn tuple_struct() {
    struct test1(i32, char, f32);
    let i = test1(12, 'A', 1.2);
    print!("{}", i.0);
}
fn struct_sample() {
    struct test {
        int: i32,
        character: char,
        float: f32,
    }
    let i = test {
        int: 12,
        character: 'M',
        float: 3.14,
    };
    print!("{}", i.float);
}
fn tuple_sample() {
    let mut tup: (i32, f64, char) = (100, 3.14, 'A');
    tup.0 = 102;
}
fn enum_with_type() {
    #[allow(dead_code)]
    enum result {
        success(u8),
        failure(u16, char),
    }
    let val = result::success(2);
    match val {
        result::success(0) => print!("0"),
        result::success(1) => print!("1"),
        result::success(n) => print!("{}", n),
        result::failure(0, 'x') => print!("3"),
        result::failure(code, module) => {
            print!("Error is n.{} of module {}", code, module);
        }
    }
}
fn enum_method() {
    #[allow(dead_code)]
    enum Contienent {
        Asia,
        Africa,
        Europe,
        America,
    }
    let mut a = Contienent::Asia;
    match a {
        Contienent::Asia => {
            a = Contienent::America;
            print!("A");
        }
        Contienent::Africa => print!("As"),
        _ => {}
    }
}
fn uSize() {
    let i: usize = 25;
    let a: f32 = 3.25;
}
fn arr_play() {
    let mut i = vec![1, 2, 3];
    i.insert(1, 12);
    println!("{}", i[1]);
}
fn play_with_arr() {
    let mut x = vec!["This", "is", "a", "sentence"];
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
    x.insert(1, "line");
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
    x.insert(2, "contains");
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
    x.remove(3);
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }
    println!();
}
fn vec_use() {
    let mut i = vec!["a", "b"];
    println!("{}", i.len());
    i.push("c");
    println!("{}", i.len());
}
fn rust_attr() {
    #[deny(unused_variables)]
    #[warn(unused_variables)]
    #[allow(unused_variables)]
    let i = 3;
}
fn if_change() {
    let i = true;
    let val = if i == true { "abc" } else { "12" };
    if i == true {
        print!("{}", val);
    }
}
fn while_Loop() {
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
}
fn for_Loop() {
    let i = 5;
    for i in 1..i {
        println!("{}", i);
    }
}
fn array() {
    let arr = ["a", "b", "c", "d", "e"];
    for i in 0..arr.len() {
        print!("{} - ", arr[i]);
    }
}