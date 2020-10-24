fn main() {
    println!("Raindrops to 100");

    let factors = vec![
        (3, "Pling".to_string()),
        (5, "Plang".to_string()),
        (7, "Plong".to_string())
    ];

    for x in 0..100 {
        let result = build_string(x, &factors);
        if result == "" {
            println!("The number is {}", x)
        } else {
            println!("The number is {} - {}", x, result)
        }

    }
}

fn parse_number(number: String) -> i32 {
    let str_number = number.to_string();
    str_number.parse::<i32>().unwrap()
}

fn build_string(number: i32, factors: &Vec<(i32, String)>) -> String {
    let owned_string = "".to_string();
    factors.iter().fold(owned_string, |s, f| {
        
        return if number % f.0 == 0 {
            [s, f.1.clone()].concat()
        } else {
            s
        }
    })
}
