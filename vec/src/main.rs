fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // immutable borrow occurs here
    v.push(6);
    println!("The first element is: {}", v[0]);
}
