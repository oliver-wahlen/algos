fn main() {
    let mut s = String::from("Nihao");
    let s1 = s.clone();
    s.push_str(", didi");
    let s_len = cal_len(&s);
    //let s_len = cal_len_unsafe(s);
    println!("{s}. s1: {s1}");
    println!("{}", &s[4..]); 

}

fn cal_len(s: &String) -> usize {
    s.len()
}
fn cal_len_unsafe(s: String) -> usize {
    s.len()
}
