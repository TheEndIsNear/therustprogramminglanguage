fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");  // push_str appends a literal to a String
                             // s comes into scope

    println!("{}", s);

    takes_ownership(s);      // s's value moves into the function...
                             // .. and so is no longer valid here

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {} s2 = {}", s1, s2);

    let x = 5;               // x comes into scope
    let y = x;

    println!("x= {}, y = {}", x, y);

    makes_copy(x);          // x would move into the function,
                            // but i32 is Copy, so it's okay to
                            // still use x afterward

    let s3 = gives_ownership();   // gives_ownership moves its retun
                                  // value into s3
    let s4 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    println!("s3 = {}, s4 = {}", s3, s4);

    let (s5, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s5, len);

} // Here, x goes out of scopre, then s. But because s's value was moved,
// nothing special happens

fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {

    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
