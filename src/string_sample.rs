fn string_type() {
    let mut text: String = "H".to_string();
    text.push('e');
    text.push('l');
    text.push('l');
    text.push('o');
    text.remove(4);
    text.insert(4, '0');
    print!("{}", text);
}
fn string_as_pointer() {
    let mut i = "".to_string();
    for _ in 0..100 {
        println!("{:p} {} {}", i.as_ptr(), i.capacity(), i.len());
        i.push('a');
    }
}
fn string_diff_types() {
    let text1: String = "abc".to_string();
    let text2: &String = &text1;
    let text3: &str = &text1;
}
fn push_string() {
    let vs = ["Hello", ", ", "world", "!"];
    let mut result = String::new();
    for s in vs {
        // result = format!("{}{}", result, s);
        result.push_str(s);
        // result +=s;
    }
    print!("{}", result);
}