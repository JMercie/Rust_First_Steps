pub fn run(){
    println!("esto imprime");

    // formatting to print variables, int, strings
    println!("{} {}", 1, "joseph");

    //positional arguments
    println!("{0} {1} {0}", "joseph", "se repite");

    // name arguments  =  puedo psar variables a los placesholder
    println!("{name} {age}", name = "joseph", age = 14);

    //  placholder traits
    println!("Binary{:b} Hexadecimal {:x} Octal{:o}",10 ,10, 10);

    // debugg traits
    println!("{:?}", (12, true, "joe"));
}