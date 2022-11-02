pub fn run(){
    let name = "Muhib";
    let mut age = 21;
    println!("My Name is {} and i am {}", name, age);
    age = 38;
    println!("My Name is {} and i am {}", name, age);
    
    const ID: i32 = 001;
    
    println!("ID: {}",ID);
    
    let (my_name,my_age)= ("Muhib","21");
    
    println!("My Name is {} and i am {}", my_name, my_age);
}