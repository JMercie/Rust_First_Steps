pub fn run (){
    let _hello = "hola";


    // we must use String::from("mi string va aqui") para declarar un string con longitud variable
    let mut hola = String::from("hola esto es con una growable string");

    // println!("{}", hola.len());

    //push a char
    hola.push('w');

    //push a string 
    hola.push_str("orld");

    println!("{}", hola);

    //assertions testing

    assert_eq!(41, hola.len()); // si el test falla, rompe en el assert, si pasa compilara sin problmeas
    
}