enum Result<T, E> {
    Ok(T),
    Err(E)
}
pub fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}
pub fn show_divide(a:f64,b:f64)->Result<f64,String>{
    match divide(a, b) {
        Ok(val)=>print!("{}/{}={}",a,b,val),
        Ok(msg)=>print!("can not divide {} by {} : {}",a,b,msg),
    }
}