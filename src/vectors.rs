// vectors are resizable arrays and occupy less memory

pub fn run () {

    let mut numbers: Vec<i32>= vec![1, 2, 4,5,3];

    println!("{:?}", numbers );

    //get single value
    println!("{}", numbers[0] );

    // re assign a value

    numbers[2] = 20;

    //add on to vector

    numbers.push(5);
    numbers.push(6);

    //pop the last value

    numbers.pop();

    println!("{}", numbers.len());

    println!("{} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers;

    println!("slice {:?}", slice);

    for x in numbers.iter(){
        println!("number {}", x);
    }

    //loop & mutate values

    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("numbers vec {:?}", numbers);
}