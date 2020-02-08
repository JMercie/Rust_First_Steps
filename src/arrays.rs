// Arrays - fixed list where elements are the same data types

pub fn run () {

    let mut  numbers: [i64;5] = [1, 2, 4,5,3];

    println!("{:?}", numbers );

    //get single value
    println!("{}", numbers[0] );

    // re assign a value

    numbers[2] = 20;

    println!("{}", numbers.len());

    println!("{} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i64] = &numbers;

    println!("slice {:?}", slice);
}