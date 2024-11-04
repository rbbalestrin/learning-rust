use std::io;
fn main() {
    let mut x: i32 = 1;
    let mut y = 0;
    let mut auxiliary:i32;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Falha");
    let mut n = n.trim().parse().expect("Not a number");

    'fibonatti: while y < n{
        auxiliary = x + y;
        y = x;
        x = auxiliary;

        println!("{}", auxiliary);
    }
}

