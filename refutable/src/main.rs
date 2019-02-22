fn main() {
    // let Some(x) = Some(5); 
    // ^^^^^^^ pattern `None` not covered
    if let Some(x) = Some(5) {
        println!("Hello, world! value = {:?}", x);
    } else {
        panic!("fuck");
    }
}
