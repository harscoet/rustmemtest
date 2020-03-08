use warp::Filter;

#[tokio::main(basic_scheduler)]
async fn main() {
    let routes = warp::any().map(|| {
        run();
        "Hello World"
    });

    println!("API listening...");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

fn run() -> () {
    println!("Run..START");

    let mut value = vec![];

    for _ in 1..10000000 {
        value.push("Hello World".to_string())
    }

    println!("Run...DONE");

    ()
}
