// fn main() {
//     // let mut s = String::from("Kolya");
//     // s.push_str(", Petya");
//     // println!("{}", s);
//     // let  _n = s.clone();
//     // println!("{}", s);
//     // let x = 5;
//     // let y = x;
//     // println!("{}", x);
//     let s = String::from("petya");
//     take_ownership(s);
//     let x = 5;
//     make_copy(x);
// }
// fn take_ownership(some_string: String) {
//     println!("{}", some_string);
// }
// fn make_copy(some_value: i32) {
//     println!("{}", some_value);
// }

// fn main() {
//     let s1 = give_ownership();
//     let s2 = String::from("hello!");
//     let s3 = takes_and_gives_back(s2);
//     println!("{}, {}", s1, s3);
// }
// fn give_ownership() -> String {
//     let some_string = String::from("Hello!");
//     some_string
// }
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn main() {
    
    let my_string = String::from("hello, world");
    let word = first_word(&my_string[..]);
    let my_string_2 = "hello world!";
    let word = first_word(&my_string_2[..]);
    let word = first_word(my_string_2);
    println!("{}", word);
    
}
fn first_word(s: &str) -> &str{
    let bytes =s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn main() {
//     let s = String::from("Hello world");
//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{}, {}", hello, world);
//     let s1 = String::from("Rust is language");
//     let s2 = &s1[..3];
//     let s3 = &s1[..];
//     println!("{}\n{}", s2, s3);
// }




// fn main() {
//     let s1 = String::from("Hello!");
//     let s2 = calc_length(&s1);
//     println!("{}", s2);
//     let s3 = s1;
//     let mut  s = String::from("Hello");
//     change(&mut s);
//     println!("{}", s);
// }
// fn calc_length(s: &String) ->  usize {
//     s.len()
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", World!")
// }