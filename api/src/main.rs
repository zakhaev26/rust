#![allow(non_snake_case)]
use std::{i8,i16,i32,i64,u8,u16,u32,u64,isize,usize,f32,f64};

use std::io::stdin ;


fn main() {
    let  num: i32= 10;
    println!("Hello, world!");
    println!("the num is {} {}",num,num+2);

    let name: &str = "Bek";
    let surName: &str = "Brace";

    println!("My name is {} {} ",name,surName);

    let (country ,cap ) = ("France ", 34);

    println!("{} {} ",country,cap)
}
