// use to iterate until a condition is met

pub  fn run(){
        // let mut flag = 0;

        // loop {
        //     flag +=1;
        //     println!("{}", flag);

        //     if flag == 20{
        //         break;
        //     }
        // }

        // //while loop

        // while flag <= 100 {
        //     if flag % 15 == 0{
        //         println!("fizzbuzz");
        //     } else if flag % 3 == 0{
        //         println!("fizz");
        //     } else if flag % 5 ==0{
        //         println!("buzz");
        //     }else {
        //         println!("flag {}", flag);
        //     }
        //     flag +=1;
        // }

        // for with range
        for x in 0..100 {
            if x % 15 == 0{
                println!("fizzbuzz");
            } else if x % 3 == 0{
                println!("fizz");
            } else if x % 5 ==0{
                println!("buzz");
            }else {
                println!("flag {}", x);
            }
            
        }
}