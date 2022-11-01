pub fn run(){
    // Pringt to conlole
    println!("Hello from the print.rs file");

    println!("{}  is from {}","Brad","Mass");

    println!("{0} is from {1} and {0} likes to {2}","Brad","Mass","codes");

    println!("{name} likes to play {activity}", activity = "cricket",name = "jhone");

    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    println!("{:?}",(12,true,"hello"));

    println!("10 + 10 = {}",10+10)
}