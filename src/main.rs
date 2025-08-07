// fn main() {

    //Strings

    // let greet: String = String::from("hello World");
    // println!("{}", greet);
    // let char1: Option<char> = greet.chars().nth(1);

    // match char1 {
    //     Some(c) => println!("{}", c),
    //     None => println!("No chars"),
    // }
    // println!("{}", char1.unwrap());
    


    //Conditional statement

    // let is_even = true;

    // if is_even{
    //     println!("True");
    // }else if !is_even {
    //     println!("False")
    // }


    // for i in 0..11{
    //     println!("{}", i);
    // }

//     let sentence = String::from("saket is my name");
//     let first_word = get_first_word(sentence);
//     println!("{}", first_word);

// }

// fn get_first_word(sentence: String) -> String {
//     let mut ans = String::from("");
//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
// }








// fn main() {
    // heap_string();
    // update_string();

    // let num = 1;
    // call(num);

    // let my_string = String::from("hello");
    // let str1 = take_ownership(my_string); // moved the value

    //solutions are clone or borrow
    // let str1 = take_ownership(my_string.clone());


    // let str1 = borrow_ownership(&my_string);
    // println!("new_string: {}", my_string);
    // let new_string = my_string; // cannot move the ownership when the string is borrowed
    // println!("str: {}", str1);
    // println!("new_string: {}", new_string);
    

//     let mut s1 = String::from("hello");
//     let s2 = &mut s1;                                     // to make it update need to pass as &mut into the function
//     s2.push_str(" World");
//     println!("s2: {}", s2);                                                    
//     let s3 = &s1;                                         // u can borrow immutable reference after end of scope of &mut ref 
//     println!("s2: {}", s3);               //immutable borrow s3 would still be allowed after the first use of s2 for mutation           
//     // update_word(&mut s1);
//     // println!("s1: {}", s1);                                                 // there would only one mutable reference
//     println!("s2: {}", s2);                                                 

// }

// fn update_word(s: &mut String){
//     s.push_str(" World");
// }

// fn borrow_ownership(my_string: &String) -> &String{
//     return my_string;
// }

// fn take_ownership(my_string: String) -> String{
//     return my_string;
// }

// fn call(num: i32) {
//     println!("{}",num);
//     call(num+1);
// }


// fn heap_string() {
//     let s1 = String::from("hello");
//     let s2 = String::from(" world");
//     let combined = format!("{} {}", s1, s2);
//     println!("Combined : '{}'", combined);
// }

// fn update_string() {
//     let mut s = String::from("Initial string");
//     println!("Before: {}", s);
//     println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
//     s.push_str("added some additional text");
//     println!("updated: {}", s);
//     println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
// }













// Structs 

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// //by using struct we can implement function with in

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area (&self)-> u32{
//         return self.width * self.height;
//     }
// }

// // Define a tuple struct to represent a color (RGB values)
// struct Color(i32, i32, i32);
// // Define a tuple struct to represent a 3D point
// struct Point(i32, i32, i32);



// // Unit Structs
// // Define a trait that requires a 'show' method
// trait Show {
//     fn show(&self);
// }

// // Define a unit struct named Line
// struct Line;

// // Implement the Show trait for the Line unit struct
// impl Show for Line {
//     fn show(&self) {
//         println!("This is a line!");
//     }
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("saketpandey"),
//         email: String::from("saketpandey@gmail.com"),
//         sign_in_count: 8
//     };
//     println!("{} sign in count is {}", user1.username, user1.sign_in_count);


//     let rect = Rect {
//         width: 30,
//         height: 20
//     };
//     println!("area of rect: {}", rect.area());


//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);
//     println!("Origin coordinates: ({}, {}, {})", origin.0, origin.1, origin.2);

//     let my_line = Line; // Create an instance of the unit struct
//     my_line.show(); // Call the method implemented for the unit struct
// }





// //Enums & Pattern Matching
// enum Directions {
//     North,
//     West,
//     South, 
//     East
// }

// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64)
// }
// fn cal_area(shape: Shape)-> f64{
//     let ans = match shape {
//         Shape::Circle(r) => std::f64::consts::PI * r * r,
//         Shape::Square(s) => s * s,
//         Shape::Rectangle(l, b) => l * b
//     };
//     return ans;
// }










// use std::fs;


// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


// fn main() {
    // let my_dir = Directions::North;
    
    // let circle = Shape::Circle(5.0);
    // let square = Shape::Square(5.0);
    // let Rect = Shape::Rectangle(5.0, 5.0);
    
    // let res = fs::read_to_string("example.txt");
    
    // match res {
    //     Ok(content) => {
    //         println!("File content: {}", content);
    //     },
    //     Err(e) => {
    //         println!("Error reading file: {}", e);
    //     }
    // }



//     try_division(4, 2);
//     try_division(1, 0);

//     // Binding `None` to a variable needs to be type annotated
//     let none: Option<i32> = None;
//     let _equivalent_none = None::<i32>;

//     let optional_float = Some(0f32);

//     // Unwrapping a `Some` variant will extract the value wrapped.
//     println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

//     // Unwrapping a `None` variant will `panic!`
//     println!("{:?} unwraps to {:?}", none, none.unwrap());
// }

// // An integer division that doesn't `panic!`
// fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
//     if divisor == 0 {
//         // Failure is represented as the `None` variant
//         None
//     } else {
//         // Result is wrapped in a `Some` variant
//         Some(dividend / divisor)
//     }
// }

// // This function handles a division that may not succeed
// fn try_division(dividend: i32, divisor: i32) {
//     // `Option` values can be pattern matched, just like other enums
//     match checked_division(dividend, divisor) {
//         None => println!("{} / {} failed!", dividend, divisor),
//         Some(quotient) => {
//             println!("{} / {} = {}", dividend, divisor, quotient)
//         },
//     }
// }





// use chrono::{Local, Utc};

// fn main() {
//     // Get the current date and time in the local timezone
//     let local_time = Local::now();
//     println!("Local time: {}", local_time);

//     // Get the current date and time in UTC
//     let utc_time = Utc::now();
//     println!("UTC time: {}", utc_time);

//     // Format the local time as a string
//     let formatted_local = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
//     println!("Formatted Local Time: {}", formatted_local);
// }


















//Collections





// --> Vectors

// fn main(){
// let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);

//     println!("Vector: {:?}", vec);
//     let even_numbers = even_filter(&vec); 
//     println!("Vector: {:?}", even_numbers);
//     println!("Vector: {:?}", vec);


//     let num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let even_numbers = even_filter(&num);
//     println!("Even numbers: {:?}", even_numbers);
// }


// fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(*val);
//         }
//     }
//     return  new_vec
// }










//HashMaps

// use std::collections::HashMap;
// fn main() {
//     let mut users = HashMap::new();

//     users.insert(String::from("saket"), 20);
//     users.insert(String::from("sachin"), 21);
//     users.insert(String::from("sanjay"), 22);

//     let first_user = users .get("saket");

//     match first_user {
//         Some(age) => println!("saket's age is {}", age),
//         None => println!("User not found"),
//     }



//     let inp_vec = vec![(String::from("saket"), 20), (String::from("sachin"), 21), (String::from("sanjay"), 22)];
//     let hm = group_values_by_key(inp_vec);
//     println!("Grouped HashMap: {:?}", hm);
// }

// fn group_values_by_key(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
//     let mut hm = HashMap::new();
//     // for (key, value) in pairs {
//     //     map.entry(key).or_insert_with(Vec::new).push(value);
//     // }

//     for (key, value) in vec {
//         hm.insert(key, value);
//     }
//     hm
// }












//Iterators

// fn main() {
    // let nums = vec![1,2,3];
    // let iter = nums.iter();
    // for value in iter {
    //     println!("{}", value);
    // }



    // //iter_mut

    // let mut v1 = vec![1, 2, 3];
    // // let v1_iter = v1.iter_mut();
    // let mut v1_iter = v1.iter_mut();

    // // for val in v1_iter {
    // //     *val = *val + 1
    // // }
    
    // while let Some(val) = v1_iter.next(){
    //     println!("{}", val);
    // }
    
    // println!("{:?}", v1);




    //into_iter

    // let nums = vec![1, 2, 3];
    // let iter = nums.into_iter();

    // for value in iter {
    //     println!("{}", value);
    // }


    //map
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter();
//     // let v1_iter2 = v1_iter.map(|x| x + 1);
//     // let v1_iter2 = v1_iter.filter(|x| *x % 2 == 0);
//     let v1_iter2 = v1_iter.filter(|x| *x % 2 == 1).map(|x| x + 1);
//     // for val in v1_iter2 {
//     //     println!("{}", val);
//     // }

//     let v2: Vec<i32> = v1_iter2.collect();
//     println!("{:?}", v2);
// }










//      String & Slices

// fn main (){

//     let mut word = String::from("Hello world");
//     // let word2 = &word[0..5];
//     // word.clear();
//     // let word2 = find_first_word(word);                                  //Here we are transferring ownership that causing error
 
//     let word2 = find_first_word(&word);                                  

//     println!("{}", word2);

// }


// fn find_first_word(word: String) -> &str {
// fn find_first_word(word: &String) -> &str {
//     let mut index = 0;
//     for (_, i) in word.chars().enumerate() {
//         if i == ' ' {
//             break;
//         }
//         index = index + 1;
//     }
//     return &word[0..index];
// }














// Generics

// fn main() {
//     let bigger = largest(1, 2);
//     println!("{}", bigger);
//     let bigger_char = largest('a', 'b');
//     println!("{}", bigger_char);
// }

// fn largest<T: std::cmp::PartialOrd>(a: T, b: T)-> T {
//     if a > b {
//         a
//     }else {
//         b
//     }
// }










//Traits

// use std::fmt::format;

// pub trait Summary {
//     fn summarize(&self) -> String {
//         return String::from("Summary not implemented");
//     }
// }
// pub trait Fix {
//     fn fix(&self) -> String {
//         return String::from("Fix not implemented");
//     }
// }

// struct User {
//     name: String,
//     age: u32,
// }

// // impl Summary for User {
// //     fn summarize(&self) -> String {
// //         return format!("User {} is {} years old", self.name, self.age);
// //     }
// // }


// // pub fn notify(item: &impl Summary) {
// //     println!("Breaking news! {}", item.summarize());
// // }


// impl Summary for User {}
// impl Fix for User {}


// fn main(){
//     let user = User{
//         name: String::from("saket"),
//         age: 20
//     };
//     // notify(&user);
//     // println!("{}", user.summarize());
//     notify(user);
// }


// fn notify<T: Summary + Fix>(u: T) {
//     println!("Breaking news! {}", u.summarize());
// }










//Lifetime


// struct User {
//     name: &str,
// }
// struct User<'a> {
//     name: &'a str,
// }

// fn main() {
    // let longest_str;
    // let str1 = String::from("hello");
    // {
    //     let str2 = String::from("world");
    //     longest_str = longest(&str1, &str2);
    // }
    // println!("{}", longest_str);

//     let name = String::from("saket");
//     let user = User {name: &name};
//     println!("{}", user.name);
// }

// fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         return str1;
//     }else {
//         return str2;
//     }
// }









// MultiThreading

// use std::thread;
// use std::time::Duration;

// fn main(){
    // let handle = thread::spawn(||{
    //     for i in 0..10{
    //         println!("spawned thread {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();                                     // wait for the spawned thread to finish
    // println!("After spawned thread");

    // for i in 0..10{
    //     println!("main thread {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

//     let x = 4;
//     {
//         let v = vec![1,2,3,4,5,6,7,8,9,10];
//         thread::spawn(move || {                                    //transfer ownership of v to the spawned thread
//             println!("Here is the vector: {:?}", v);
//         });
//     }
//     handle.join().unwrap();
//     println!("x is {}", x);
// }









//Message passing
use std::sync::mpsc;                                                                // multiple producers, single consumer
use std::thread::spawn;

fn main(){
    // let (tx, rx) = mpsc::channel();
    // spawn(move || {
    //     tx.send(String::from("Hello")).unwrap();
    // });
    // let value= rx.recv().unwrap();
    // println!("{}", value);


    let (tx, rx) = mpsc::channel();

    for i in 0..10{
        let producer = tx.clone();
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000..(i + 1 * 10000000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);

    let mut final_sum = 0;
    for val in rx {
        println!("Receiving value from channel: {}", val);
        final_sum = final_sum + val;
    }
    println!("Final sum: {}", final_sum);

}