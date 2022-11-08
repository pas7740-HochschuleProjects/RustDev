fn main() {
    let mut num = 5;
    num = double_value(num);
    println!("Number is: {num}");
}

fn double_value(x:i8)->i8{
    let y = x*2;
    return y;
}
