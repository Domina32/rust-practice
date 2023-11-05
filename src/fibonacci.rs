use std::collections::HashMap;

fn fib(num: u64, mut cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(result) = cache.get(&num) {
        return *result;
    }

    if num == 0 {
        cache.insert(0, 0);
        return 0;
    }
    if num == 1 {
        cache.insert(1, 1);
        return 1;
    }
    if num == 2 {
        cache.insert(2, 2);
        return 2;
    }

    let n2 = fib(num - 2, &mut cache);
    cache.insert(num - 2, n2);

    let n1 = fib(num - 1, &mut cache);
    cache.insert(num - 1, n1);

    return n1 + n2;
}

pub fn nth_fibonacci() -> Result<(), Box<dyn std::error::Error>> {
    println!("calculate nth fibonacci for n=");
    let mut n = String::from("");
    std::io::stdin().read_line(&mut n)?;

    let mut cache: HashMap<u64, u64> = std::collections::HashMap::default();
    let num = n.as_str().replace("\n", "").parse().unwrap_or_default();

    println!("nth fibonacci number is: {}", fib(num, &mut cache));

    Ok(())
}

pub fn nth_fibonacci_loop() -> Result<(), Box<dyn std::error::Error>> {
    println!("loop time!");
    println!("calculate nth fibonacci for n=");
    let mut n = String::from("");
    std::io::stdin().read_line(&mut n)?;

    let mut cache: HashMap<u64, u64> = std::collections::HashMap::default();
    let num: u64 = n.as_str().replace("\n", "").parse().unwrap_or_default();

    let mut nth_fib_num: u64 = 1;

    for i in 1..(num + 1) {
        if i == 1 {
            cache.insert(1, 1);
            nth_fib_num = 1;
        } else if i == 2 {
            cache.insert(2, 2);
            nth_fib_num = 2;
        } else {
            nth_fib_num = cache.get(&(i - 2)).unwrap_or(&0) + cache.get(&(i - 1)).unwrap_or(&0);
            cache.insert(i, nth_fib_num);
        }
    }

    println!("nth fibonacci number is: {}", nth_fib_num);
    Ok(())
}
