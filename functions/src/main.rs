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
impl PermissionLevel {
    fn description (&self)->String{
        //match is similar to a switch statement must match all enum
        match self{
            PermissionLevel::Admin=>String::from("I am Admin"),
            PermissionLevel::Instructor=>String::from("I am Instructor"),
            PermissionLevel::User=>String::from("I am User"),

        }
    }//this is a method 
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
        }
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