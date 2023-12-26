#![allow(non_snake_case)]
use std::{f32, f64, i16, i32, i64, i8, isize, u16, u32, u64, u8, usize};

use std::io::stdin;

fn main() {
    let num: i32 = 10;
    println!("Hello, world!");
    println!("the num is {} {}", num, num + 2);
    let name: &str = "Bek";
    let surName: &str = "Brace";
    println!("My name is {} {} ", name, surName);
    let (country, cap) = ("France ", 34);
    println!("{} {} ", country, cap);
    let _num = 34;

    let mut x = 15;

    if x <= 20 {
        println!("You are less than 20");
    } else if x < 30 {
        println!("You are gt 20 lt 30");
    } else {
        println!("You old Dawg");
    }

    // loop {
    //      x +=1;

    //     if x % 2 == 0 {
    //         continue;
    //     }
    //     println!("{}",x);

    //     if x >=20000 {
    //         break
    //     }
    // }
    let xs = 1..6;


    for i in 1..=100 {
        println!("Number is {}", i)
    }

    for val in xs {
            println!("{}",val)
    }


    
    let fruits = vec!["Gay","LGBTQIA+","SIGMA"];

    for fruit in fruits.iter() {
        println!("Borrowed: {}", fruit);
    }

    for fruit in &fruits {
        println!("Borrowed: {}", fruit);
    }

    println!("Vector after Borrow: {:?}", fruits);

    // Using into_iter() - Consuming ownership
    for fruit in fruits.into_iter() {
        println!("Owned: {}", fruit);
    }

    // The vector cannot be used here because into_iter() consumed ownership
    
    // this line would result in a compilation error:
    // println!("Vector after Getting Owned: {:?}", fruits);    
    let mut horn :i32 = 23;

    while horn < 90 {
        println!("{}",horn);
        horn+=1;
    }


    let mut tup: (&str,char, i32) = ("name",'c',100);
    println!("{:?}",tup.2);

    tup.2 = 101
}
