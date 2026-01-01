use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1); 
    v.push(2); 
    v.push(3);
    let mut v1 = vec![1,2,3];
    v1.push(4);
    let last: &i32 = &v[2];
    println!("{}", last);
    let last1: Option<&i32> = v.get(3);
    match last1{
        Some(third) => println!("{}", third),
        None => println!("Does not exist"),
    }

    for i in &mut v1 {
        *i *= 10;
        println!("{}", i);
    }

    let mut s = "你好".to_string();
    s.push_str("， 我的名字是安娜");
    let s_cannibalized = String::from("Hello");
    let s_survive = String::from(", there");
    let s_new = s_cannibalized + &s_survive;
    println!("{s_survive} {s_new}");
    let s_format = format!("{s_new}§{s_survive}");
    println!("{} {}", s_new, s_format);
    println!("{} {}", &s[0..6], &s_new[0..1]);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 100);
    scores.insert(String::from("Yellow"), 200);

    let team = "Blue".to_string();
    let score = scores.get(&team).copied().unwrap_or(0);

    println!("{score}");
}
