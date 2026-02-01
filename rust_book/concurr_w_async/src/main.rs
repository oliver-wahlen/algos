use std::time::Duration;
extern crate trpl;
use trpl::StreamExt;
use std::pin::{Pin, pin};

fn main() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Num {i} from first task");
                trpl::sleep(Duration::from_millis(50)).await;
            }
        });

        for i in 1..5 {
            println!("Hi num {i} from second task");
            trpl::sleep(Duration::from_millis(50)).await;
        }

        handle.await.unwrap();

        println!("---------------");
        let fut1=async {
            for i in 1..10 {
                println!("Hi num {i} from 1_task");
                trpl::sleep(Duration::from_millis(50)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("Hi num {i} from 2_task");
                trpl::sleep(Duration::from_millis(50)).await;
            }
        };
        
        trpl::join(fut1, fut2).await;
    });

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx_fut = pin!(async move {

            let vals = vec![
                String::from("hi"),
                String::from("nihao"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(50)).await;
            }
        });
        
        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("rec: {value}");
            }
        });

        let tx1_fut = pin!(async move {
            // --snip--
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(50)).await;
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
        trpl::join_all(futures).await;
    });



    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n*2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("val: {value}");
        }
    });
}
