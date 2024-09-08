

// ! Mutabiltiy implementation

fn main() {
    let mut num: i32 = 3;
    num += 1;
    println!("{}", num);
    
    let mut s = String::from("Mutable string");  // String is heap-allocated and growable
    println!("{}", s);  // Use `s` to avoid the warning
    s = String::from("New string value");  // Reassigning a new value to `s`

    let mut s2 = String::from("Mutable string slice");
    s2.clear();
    s2.push_str("Completely new content");

    println!("{}  {}", s, s2);
}

// ! Function implementation

    // let mut num: i32 = 3;
    // 
    // for _i in 0..5 {
            // num = num + 1;
        // println!("{}", num);   
        // }
        // 
        // if num > 5 {
            // println!("It is really greater then 5 and the value is {}",num);
        // }
        // else{
            // println!("It is not {}",num)
        // }
        // addition (3,num);
        // addition (2,num);
// 
        // let mut _data_owning_string: String = String::from("The data_owning_string is heap stored, mutable, owned by declaration ");
// 
        // let _string_slice: &str = "string_slice is reffrenced saved and is also used for borrowing part of string";
    // 
    // 
//    
    // 
    // pub fn addition (a:i32,b:i32) {
        // println!("The Sum is {}",a+b);
    // }
    
// ! String implementation

//     let name: String = String::from("John"); // This way of initialization is known as a string literal
//     let string2: &str = "Hello, world!"; // This way of initialization is known as a string slice || reference to a string
//     println!("Hello, {}!", name);
//     println!("Hello, {}!", string2);


   // Since the char1 at index 1000 might not be char so printing using println! will not work
   // So we need to use match to check if the char is Some or None
   // If the char is Some then we can print it
   // If the char is None then we can print a message
   // ! This is how rust save us from static type checking for char in future tense
//     let char1: Option<char> = string2.chars().nth(1000);

//     match char1 {
//         Some(c) => println!("The char is {}", c),
//         None => println!("The char is None"),
//     }
    

// ! IF else implementation

    // println!("Hello, world!");
    // let is_male: bool = true;
    // let is_female: bool = false;
    // let is_above_18: bool = true;

    // if is_male{
    //     println!("You are a man");
    // }
    // else if is_female && is_above_18{
    //     println!("You are a woman is above 18");
    // }

