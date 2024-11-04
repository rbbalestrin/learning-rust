

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

fn memory_management(){
    let b = Box::new([0; 100]);
    free(b); //imaginary function that deallocate memory
    assert!(b[0] == 0); //Will cause a error because its trying to use a pointer that dont exist anymore
}

fn make_and_drop() {
    let a_box = Box::new(5);
} //When this function ends, Rust will deallocate the variable frame (because its in the Stack)
// And when the frame is "killed" the heap is also deallocated


//So... What happens with this?
fn box_and_owner() {
    let a = Box::new(5);
    let b = a;
    // When Rust would try to free the box's heap memory, it would try twice,
    // and that is undefined behaviour.
}
fn box_and_owner2(){
    let a = Box::new(5); // To avoid this situation, Rust make the first variable **a, owner of the box**
    let b = a;  // And when let b = a, Rust moves the ownership,
    // so when the scopes ends, b is deallocated, not a
    // When with this, if the variable is used to something
    let c = b + 1;
    // The heap memory is changed, making both a and b, invalid because they are pointing to "nothing"
}