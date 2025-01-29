mod basket;
mod stack;
mod container;

use basket::Basket;
use num_traits::{Float, ToPrimitive};
use container::Container;
use stack::Stack;
/*
Notes on traits:
- Set of methods
- Can contain abstract methods i.e not implementation
- Can contain default methods which do have an implementation
 */

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}
fn non_generic_solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {// Float is referred to as a trait and is being used as a trait bound
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    //Standard use case for generics ... rust does not do automatic type conversion
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    let a_f64 = a as f64;
    let a_f64 = a.to_f64().unwrap();

    println!("{}", solve(a_f64, b));

    let mut b1 = Basket::new(String::from("hi there"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("hi")]);
    let s2 = Stack::new(vec![1, 2, 3]);

    //Demoing trait bounds
    add_string(&mut b1, String::from("hi"));
    add_string(&mut s1, String::from("hi"));

}