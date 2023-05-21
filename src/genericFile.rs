use std::string;

fn swap<T1,T2>(a:T1,b:T2)->(T2,T1){(b,a)}
pub fn use_swap(){
    let x = swap::<char,i32>('a',32);
    print!("{}",x.0);
}
enum option<T>{
    some(T),
    None
}
pub fn arr_pop(){
    let arr = vec![1,2,3,4,5];
    for _ in 0..7 {
        let i:option<i32>=arr.pop();
        match i {
            Some(number)=>print!("{}",number),
            None=>print!("#"),
        }
    }
}