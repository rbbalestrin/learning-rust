fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2); //Because its only a pointer to a string, when the function ends
    // we can still use m1 and m2
    let s = format!("{} {}", m1, m2);
    println!("{}", s);
    println!("---------");
    //Dereferencing
    let mut x = Box::new(3); // x points to heap which have 1
    let a = *x; //*x read the heap value, so a = 1
    *x += 1; //dereference the x to increse the heap value to 2, it doesnt change the value of a

    let r1 = &x; //pointing to x in the stack
    let b = **r1; //two dereferences, so get the heap value, b = 2

    let r2 = &*x; //r2 goes to x, and then dereferences, in the end pointing to the value
    let c = *r2; //the same thing about a
    *x += 1;
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}", g1, g2);
}

fn borrow_checker() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);
    //It will cause a error, because we are alising the data in num
    // and trying to mutate with push
    println!("Third element is {}", *num);
}

fn mutable_references() {
    let mut v = vec![1, 2, 3];
    let num = &mut v[2];
    *num += 1; //Dereference the num so it goes to 2 and increses to 3
    println!("Third element is {}", *num); //After this num is no longer in use, v regains all permissions

    println!("Vector is now {:?}", v);
}