fn main() {
    let x :i32 = 5; // if unint but used //err
    let _y :i32; //unit but also unused //warning
    assert_eq!(x,5);
    println!("Sucess");
}

//use _y to fix above...


//Mut Keyword
 fn main() {
	let mut x :i32  = 1 ; //by default variables are immutable in rust.to allow mutation,u must use mut keyword.
	x+=2;
	assert_eq!(x,3);
	println("Success!");
}

//Scopes
 fn main() {
	let x :i32 = 10;
	let y :i32 = 15;
	{
		let y:i32 =5;
		println!("The value of x is {} and value of y is {}",x,y); //10,5
	}
	println!("The value of x is {} and value of y is {}",x,y); // 10,15

}

// define x

fn main() {
	define_x();
}

fn define_x() {
	let x : &str = "hello";
	println!("{},world!",x);
}


//scoping 

fn main() {
	
	let x :i32 = 5;
	{
		let x = 12;
		assert_eq!(x,12);
	}
	assert_eq!(x,5);
	println!("Success!");
	
}

//destructuring tuples
fn main() {
	
	let (mut x,y) = (2,3);
	x+=2;
	assert_eq!(x,4);
	assert_eq!(y,3);	
}

fn main(){
	#[allow(unused_variables)] //allow unused variables in this block
	let (x,y);
	//you can also do let(_x,_y);
	(x,..) = (3,4);
	[..,y] = [1,2];
	
	assert_eq!([x,y],[3,2]);


}

