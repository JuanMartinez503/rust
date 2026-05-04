use std::mem;

fn main() {
    // tuples have a fixed length and can have different dtypes
    let special_tpl = ();//an empty tuple is called a unit
    let mut tpl = ( 500, "Juan",true);

//a tuple cannot print to the console unless it has a :? 
println!("{tpl:?}"); //std::fmt::Debug
let (mut x , y , z) = tpl; //this is how to discustruct a tuple
println!("{x} ,{y}, {z} "); //std::fmt::Display

x =-100; //this changes the value of x but it win't change the tuple itself
println!("{x} ,{y}, {z} "); //std::fmt::Display
tpl.0 =10000; //doing it this way will change the tuple
println!("{tpl:?}"); //std::fmt::Debug

//an array has a fixed length and only stores same dtypes
 let array = [1,2 ,3]; //saves as a stack which is faster for the computer
println!("{array:?}"); //std::fmt::Debug
let[a1,a2,a3]=array; //this is how to deconstruct an array
println!("{a1} ,{a2}, {a3} "); //std::fmt::Display
println!("{}",array[0]);//this is how you print an array index
//changing the array by index can change the value of a tuple
let arr2 = [500;5]; //cleaner an easier way to write, will print 500, 5 times
println!("{arr2:?}"); //std::fmt::Debug

let xs = [1,2,3,4,5];
println!("length of xs: {}", xs.len());
println!("Data size of xs: {}", mem::size_of_val(&xs));
//the & refers to something already existing
let slice_xs = &xs[1..2];
println!("Slice of xs {:?}", slice_xs);
println!("Slice of xs {:?}", &xs[1..=2]);//the equals includes the index
println!("Slice of xs {:?}", &xs[1..4]);
//the slices cannot change the value of the array - read only

let mut my_string = String::from("I ❤️ Rust");
println!("my_string: {}",my_string); //strings stores utf-8 
my_string.push('W');
println!("my_string: {}",my_string); //strings stores utf-8 
my_string.pop();//strings are dynamic is size
println!("my_string: {}",my_string); //strings stores utf-8 
//in rust it is not possible to access the value of a string by indexing, since the heart emoji takes 4 index for example
for byte in my_string.bytes(){
    println!("{byte}")
    //prints bytes even though there are only 7 characters but prints 11 different ones because of the heart
}
for chr in my_string.chars(){
    println!("{chr}")
}
for (pos,chr) in my_string.char_indices(){
    println!("{pos} : {chr}")
}


}
