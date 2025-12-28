fn main() {
    println!("Hello, world!");
    another_func(65.1);
    let y = {
        let a = 45;
        a
    };
    println!("{y}");
    println!("{}", mod_3(19));

    let cond = 4;
    println!("{}", if cond<2 {"less 2"} else {"larger 2"});
    'da_for_loop: for i in 1..7 {
        println!("{i}");
        let mut count=0;
        loop {
            count +=1;
            println!("C: {count}");
            if count*i>100 {
                println!("Break da loop");
                break 'da_for_loop;
            } else if count > 25 {
                break;
            }
        }
    }
}

fn mod_3(x: i32) -> i32 {
    x%3
}

fn another_func(x: f64) {
    println!("Another func");
    println!("The var is {x}");
}
