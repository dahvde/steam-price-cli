mod args;
mod item;
mod util;

use args::CsgoItemArgs;
use clap::Parser;
use item::{Item, ItemCollection};
use tokio::task;
use util::{bin_dir_file, read_lines};

#[tokio::main]
async fn main() {
    let args = CsgoItemArgs::parse();

    let dir = match args.input.as_deref() {
        Some(e) => e.to_path_buf(),
        None => bin_dir_file("items.txt").unwrap(),
    };

    let mut data: Vec<String> = read_lines(dir);
    data.retain(|x| !x.is_empty() && !x.starts_with("$~"));

    let mut handles = vec![];

    let task_size = data.len();
    let time = std::time::Instant::now();

    println!("[*] Fetching");

    // Multi-threaded requests
    for x in data {
        let handle = task::spawn_blocking(move || {
            tokio::runtime::Handle::current().block_on(Item::init(x.as_str()))
        });

        handles.push(handle);
    }

    let mut items: Vec<Item> = Vec::with_capacity(task_size);
    for handle in handles {
        let result = handle.await.unwrap();
        items.push(result.expect("balls"));
    }

    println!("[*] Done! {:.2?}", time.elapsed());
    
    let item_coll = ItemCollection::init(&items);
    item_coll.table_print();
}
