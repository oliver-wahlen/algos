use trpl::{Either,Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
            .select_first("title")
            .map(|title| title.inner_html());
    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let title_fut1 = page_title(&args[1]);
        let title_fut2 = page_title(&args[2]);

        let (url, maybe_title) = 
            match trpl::select(title_fut1, title_fut2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        println!("{url} was first");
      
        match maybe_title {
            Some(title) => println!("Title {url} was {title}"),
            None => println!("{url} no title"),
        }
    })
}
