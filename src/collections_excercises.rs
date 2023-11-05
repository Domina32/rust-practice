const VOWELS: &str = "aeiouAEIOU";

fn median(mut numbers: Vec<i32>) -> f32 {
    numbers.sort();
    if numbers.len() % 2 == 0 {
        return (numbers[numbers.len() / 2] + numbers[numbers.len() / 2 - 1]) as f32 / 2.0;
    } else {
        return numbers[numbers.len() / 2] as f32;
    }
}

fn mode(numbers: Vec<i32>) -> i32 {
    let mut hashtable: std::collections::HashMap<i32, u64> = std::collections::HashMap::new();
    for number in numbers {
        let count = hashtable.entry(number).or_insert(0);
        *count += 1;
    }
    *hashtable
        .iter()
        .max_by(|current, next| return current.1.cmp(next.1))
        .unwrap()
        .0
}

pub fn middles() {
    println!("median (1) or mode (2) ?");
    let mut middle_choice: String = String::new();
    std::io::stdin().read_line(&mut middle_choice);
    middle_choice.pop();

    println!("enter list length:");
    let mut list_length: String = String::new();
    std::io::stdin().read_line(&mut list_length);
    list_length.pop();

    println!("enter numbers:");
    let mut numbers: Vec<i32> = Vec::new();
    for _ in 0..list_length.as_str().parse().unwrap_or_default() {
        let mut number: String = String::new();
        std::io::stdin().read_line(&mut number);
        number.pop();
        numbers.push(number.as_str().parse().unwrap_or_default());
    }

    if middle_choice == "1" {
        println!("median is={}", median(numbers));
        return;
    } else if middle_choice == "2" {
        println!("mode is={}", mode(numbers));
        return;
    } else {
        println!("type 1 or 2");
        return;
    }
}

pub fn pig_latin() {
    println!("enter word:");
    let mut word: String = String::new();
    std::io::stdin().read_line(&mut word);
    word.pop();

    let mut new_word: String = String::from(&word);
    if VOWELS.contains(word.chars().nth(1).unwrap()) {
        let first_letter = word.chars().nth(0).unwrap();
        new_word = word.chars().skip(1).collect();
        new_word.push_str("-");
        new_word.push(first_letter);
        new_word.push_str("ay");
    } else {
        new_word.push_str("-hay");
    }
    println!("translation is:{}", new_word);
}
