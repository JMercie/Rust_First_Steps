pub fn run(){
    let name = "joseph";
    let mut age = 22;

    // if I use mut next to "let" then the variable is mutable;
    age = 21;

    println!("my name is {} and i'm {}", name, age);

    //define const

    const ID: i32 = 0001;

    println!("{}", ID);

    //assign multple variables

    let (mi_nombre, mi_edad) = ("joseph", 22);

    println!("{} {}", mi_nombre, mi_edad);
}