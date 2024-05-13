pub fn execute() {
    let mut sum : u32 = 0;
    let mut i = 0;
    let mut count = 0;
    loop {
        if i % 2 == 1 {
            println!("get: {:?}", i);
            let odd_num : u32 = i;
            sum += odd_num.pow(2);
            println!("squared: {:?}", odd_num.pow(2));
            count += 1;
            println!("count: {:?}", count);
            if count >= 100 {
                break;
            }
        }
        println!("loop: {:?}", i);
        i+=1;
    }
    println!("Answer: {:?}", f64::powf(sum as f64, 1.0/5.0));
}