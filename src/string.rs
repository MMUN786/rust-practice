pub fn run(){
    let mut hello = String::from("Hello");

    println!("Length: {}",hello.len());
    
    hello.push('5');
    
    println!("{}",hello);
    
}