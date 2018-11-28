use std::io;

fn main() {
    println!("Введите температуру в градусах Фаренгейта: >");
    let mut farengheit = String::new();
    io::stdin().read_line(&mut farengheit).expect("Failed to read line");
    let farengheit: f32 = farengheit.trim().parse().expect("Это не число");
    println!("{} градусов Фаренгейта будет равно {} градусам Цельсия",farengheit, far_to_cels(farengheit) );

    fibo(10);
}

fn far_to_cels(farengheit: f32) -> f32 {
    (farengheit - 32.0) * 0.555
}

fn fibo(x: u32) {
    println!("FIBONACCI SEQUENCE:");
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    let mut count: u32 = 0;
    while count < x {
        count += 1;
        let temp = a;
        a = b;
        b = temp + b;
        println!("{}", a);
    }
    println!("END OF FIBONACCI SEQUENCE!");
}