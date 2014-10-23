fn test() {
	println!("Hello, world!")
}

fn test1() {
    let mut y = 6i;
    let mut x = 100i;
    let mut z = &mut y;
    println!("z{:p}", z);
    println!("z{}", *z);
    change(z);
    //changeref(z, &mut x);
    println!("z{}", *z);
    println!("z{:p}", z);
}

fn change(z: &mut int) {
   *z = *z + 1;
}

//fn changeref<'a>(z: &'a mut int, x: &'a mut int) {
//    z = x;
//}

fn add_one(x: &int) -> int {
    *x + 1
}

fn main() {
    println!("Hello, world!")
	println!("{}",test())
    test1();
    let x = box 5i;
    println!("box = {}", add_one(&*x));
    println!("box = {}", add_one(&*x));
    println!("box = {}", add_one(&*x));
    hello::print_hello();
    let add_one = |x: int| -> int { x + 1i };
    let p = || {println!("{}", x)};
    p();
}

mod hello {
    pub fn print_hello() {
        println!("Hello World!")
    }

}

