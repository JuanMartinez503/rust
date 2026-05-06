#[derive(Debug)]
struct User {
    //struct is the same as a class 
    //main difference a enum says this thing can be one nof this , struct says this thing has these variables
    is_admin:bool,
    username:String,
    password:String
}
#[derive(Debug)]
struct RGB(i32,i32,i32); //this is a struct for a tuple
struct Circle{
    radius:f32
}
impl Circle{
    fn computer_area(&self)->f32{
        self.radius * self.radius *3.14
    }
        fn computer_circum(&self)->f32{
        2.0* self.radius *3.14
    }
    fn smaller(&self, other:&Self)->bool{
        self.radius < other.radius
    }

}
impl User{
    fn build_admin(username:String , password:String)->User {
User{
    is_admin:true,
    username,
    password
}
    
}
}
fn first_word(s: &String) -> &str {
    for (idx, char) in s.char_indices() {
        if char == ' ' {
            return &s[..idx];
        }
    }

    &s
}

fn main() {
    let s = String::from("Jan Schaffranek");

    let slice1 = &s[0..3];
    let slice2 = &s[4..];

    println!("{slice1}");
    println!("{slice2}");

    let word1 = first_word(&s);
    println!("{word1}");
    let user:User = User{
        is_admin:true,
        username:String::from("Juan"),
        password:String::from("secret")
    };
    println!("{user:?}");

    let sarah = User::build_admin(String::from("Sarah"), String::from(("pipcy")));

    println!("{sarah:#?}"); //the # makes it look prettier in the console

    // struct_methods();
    let black = RGB(0,0,0);
    println!("{black:?}");
    // closure_example();
    function_pointer();

}

fn struct_methods() {
    let c1 = Circle{radius:2.3};
    let c2 = Circle{radius:5.3};

    println!("Circle 1: {}",c1.computer_area());
    println!("Circle 1: {}",c1.computer_circum());
    println!("Circle 1: {}",c1.smaller(&c2));

}
fn closure_example() {
    let num =2;//closure helps to put the values locally
    let closure = |inp:i32|->i32 {inp*2};
    print!("closure : {}", closure(num))
}
fn function_pointer() {
    fn my_function(inp:i32)->i32 {
        inp*2
        
    }
    
fn takes_fn_as_input(f:fn(i32)->i32) {
 f(2);   
}    
println!("my_func : {:?}", takes_fn_as_input(my_function))
}