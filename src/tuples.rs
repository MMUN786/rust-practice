use std::i8;



pub fn run (){
    let person : (&str,&str,i8) = ("muhib", "Mass",21);

    println!("{} is from {} and is {}", person.0,person.1,person.2)
}