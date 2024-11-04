//Variables live in Frames
fn main() {
    let n = 5; // n, 5 in the stack
    let y = plus_one(n); // Then is main(n,5) and plus_one(x,5)
    println!("{} + 1 = {}", n, y); // And ends with main(n,5; y,6)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn heap(){
    let a = Box::new([0; 1_000_000]);
    //Pointer to a place in the heap with 1_000_000 values

    let b = a; // This is a pointer to a
} //This is the heap, which holds data that can outlive a function, because it isn't "connected" to one

