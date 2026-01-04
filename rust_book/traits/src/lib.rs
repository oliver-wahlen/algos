pub trait Summary {
    fn summarize(&self) -> String {
        String::from("----")
    }
}

pub struct NewsArt {
    pub hline: String,
    pub loc: String,
    pub auth: String,
    pub cont: String,
}

impl Summary for NewsArt {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.hline, self.auth, self.loc)
    }
}

pub struct SosPost {
    pub uname: String,
    pub cont: String,
    pub reply: bool,
    pub rpost: bool,
}

impl Summary for SosPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.uname, self.cont)
    }
}

//pub fn notify<T: Summary>(item: &T)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
