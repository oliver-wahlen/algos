fn main() {
    let x=5;
    println!("{x}");
    let x=x+1;
    println!("{x}");
    {
        let x=x*2;
        println!("Inner scope {x}");
    }
    println!("{x}");
    let a: f64 = 7.0;
    let b: f64 = 2.0;
    println!("{}  {}", 5+2, 4.5/3.0);
    println!("{}  {}", a/b, 7/2);
    println!("{}", 4.5%2.0);

    let tup : (bool, f64, u8) = (true, 589.24, 2);
    let (k, l, m) = tup;
    println!("k = {k}");
    println!("{}", tup.0);
}
