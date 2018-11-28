fn main() {
    another_function(14, 88);
    println!("Five mult function (4) return: {}",five_mult(4));
}

fn another_function(x:i32, y:i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

}

fn five_mult(x: i32) -> i32 {
    x * 5
}