#[derive(Debug)]

enum PermissionLevel{
    User,
    Instructor,
    Admin
}
#[derive(Debug)]
enum LoginData  {
    None,
    Invalid,
    Not_Registered,
    Username(String)
}
// enum Option<T> {
//     Some(T),
//     None
// }
impl PermissionLevel {
    fn description (&self)->String{
        //match is similar to a switch statement must match all enum
        match self{
            PermissionLevel::Admin=>String::from("I am Admin"),
            PermissionLevel::Instructor=>String::from("I am Instructor"),
            PermissionLevel::User=>String::from("I am User"),

        }
    }//this is a method 
       fn is_admin(&self)->bool {
        match self {
            PermissionLevel::Admin=>true,
            _=>false
        }
    }
      fn is_admin2(&self)->bool {
        if let PermissionLevel::Admin  = self {
            true 
        } else {
            false
        }
    }
}





fn main() {
    let num = 32;
    let result = my_func(num);
    println!("Result: {result}");
    
    //if , else if, else are the same as Js and do not require () but they do if a && or ||
let condition = 6 %2 ==0;
let number = if condition {5} else {6};//ternary
println!("{number}");
// call_while();
// call_loop();
// enum_lesson();
// matches_example();
second_enum();
matches_example();
// option_num();
// nested_value();
while_let();
}
//the params types has always be defined

fn my_func (num:i32)->i32 //every return has to be specified
{
println!("{num}");
num*2 //the last line in Rust always has a return
}

fn call_for(){
    for num in 0..=10{
        println!("{num}")
    }
}
fn call_while(){
    let mut counter = 0;
    //you can also give a name to a while loop or any loop if they are nested 
    //you can stop it by a break and the loop name
    'while_name :while counter<10{
        counter+=1;
        println!("{counter}")
    }
}
fn call_loop(){
    let mut counter =0;
    let result = 'loop_name : loop {
        counter+=1;
        println!("{counter}");
        if counter==10{
            break 2;
        }
    };
    println!("loop result: {result}");
}

fn enum_lesson(){
    let user = PermissionLevel::Admin;
    println!("{user:?}");//the #[derive(Debug)] lets me print this
    println!("{}",user.description());

        let user = PermissionLevel::Instructor;
    println!("{user:?}");//the #[derive(Debug)] lets me print this
    println!("{}",user.description());

        let user = PermissionLevel::User;
    println!("{user:?}");//the #[derive(Debug)] lets me print this
    println!("{}",user.description())
}
fn matches_example (){
    let number = 13;

    match number {
        1=>{
            println!("one");
        },
        2|3|5|7|11=>println!("Prime"),
        _ =>println!("None")//default case
        
    }
}
fn second_enum (){
    let user_none = LoginData::None;
    println!("{user_none:?}");
        let user_invalid = LoginData::Invalid;
    println!("{user_invalid:?}");
          let user_not_registered = LoginData::Not_Registered;
    println!("{user_not_registered:?}");
          let user_admin = LoginData::Username(String::from("Juan"));
    println!("{user_admin:?}");
    
    match user_admin {
        LoginData::None|LoginData::Invalid|LoginData::Not_Registered =>{
            println!("User Unknown")
        },
        LoginData::Username(_) => println!("{user_admin:?}")
    }



}
fn option_num (){
let x:u32 = 5;
let y:Option<u32> = Some(5); //the option type takes a generic T which can be used as any 
println!("x +y = {}", add(x, y));


    fn add(x:u32 ,y:Option<u32>) ->u32 {
        //match is required to cover all the bases for the case not to fail
        match y {
            Some(y_val) =>x + y_val,
            None =>x +0
            
        }
    }
}
fn if_let() {
    

 //i had to move the function is_admin and is_admin2 to the impl to avoid errors
  
}
fn nested_value() {

    let x :Option<u32> = Some(20);
    let y :Option<u32> = Some(25);

    println!("nested value 1 ={}", add(x,y));

    let x :Option<u32> = Some(20);
    let y :Option<u32> = None;

    println!("nested value 2 ={}", add(x,y));
    

    //this option avoids the nesting which match uses which is confusing
    fn add(x: Option<u32>, y:Option<u32>)->u32 {
        let a_value = if let Some(a_val) = x {a_val} else{0};
        let b_value = if let Some(b_val) = y {b_val} else{0};

        a_value+b_value
    }
    
}
fn while_let() {

    fn while_range() {

        let mut nums = 0..=10;
        //the next() can be called on ranges and the array has to be mut
        while let Some(num) =nums.next(){
            println!("while range:{num}")
        }
    }
    fn while_array() {
        //for you to call the next() on the array, it has to be converted into an iter first
        let mut arr =[0,1,2,3].into_iter();
        while let Some(num) = arr.next()  {
            println!("while array:{num}")

            
        }
    }
    while_array();
    while_range();
    
}