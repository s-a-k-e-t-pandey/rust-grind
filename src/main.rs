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





//Enums & Pattern Matching
enum Directions {
    North,
    West,
    South, 
    East
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}
fn cal_area(shape: Shape)-> f64{
    let ans = match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Square(s) => s * s,
        Shape::Rectangle(l, b) => l * b
    };
    return ans;
}

fn main() {
    let my_dir = Directions::North;

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(5.0);
    let Rect = Shape::Rectangle(5.0, 5.0);

    
}
