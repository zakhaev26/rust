fn main() {
    let x :i32 = 5; // if unint but used //err
    let y :i32; //unit but also unused //warning
    assert_eq!(x,5);
    println!("Sucess");
}