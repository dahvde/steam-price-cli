mod args;
mod item;
mod util;

use tokio::task;
use prettytable::{Table, format};
use args::CsgoItemArgs;
use clap::Parser;
use item::Item;
use util::{bin_dir_file, read_lines};

#[macro_use] extern crate prettytable;

#[tokio::main]
async fn main() {
    let args = CsgoItemArgs::parse();
   
    // Process -input argument
    let dir = match args.input.as_deref() {
        Some(e) => e.to_path_buf(),
        None => bin_dir_file("items.txt").unwrap(),
    };

    // Read file and remove commented items
    let mut data:Vec<String> = read_lines(dir);
    data.retain(|x| !x.is_empty() && !x.starts_with("$~"));

    let mut handles = vec![];

    let task_size = data.len();
    
    let time = std::time::Instant::now();
    
    if !args.no_print {
        println!("[*] Fetching");
    }

    for x in data {
        let handle = task::spawn_blocking(move || {
            tokio::runtime::Handle::current().block_on(init_item(x.as_str()))
        });

        handles.push(handle);
    }

    let mut items: Vec<Item> = Vec::with_capacity(task_size);
    for handle in handles {
        let result = handle.await.unwrap();
        items.push(result);
    }

    if !args.no_print {
        println!("[*] Done! {:.2?}", time.elapsed());
    }

    let table = table_format(items);

    if let Some(e) = args.output.as_deref() {
        let csv_writer = csv::Writer::from_path(e);

        table.to_csv_writer(match csv_writer {
            Ok(e) => e,
            Err(e) => panic!("Problem writing file {:?}", e),
        }).ok();
    }

    if !args.no_print {
        table.printstd();
    }
}

async fn init_item(str: &str) -> Item {
    let left: Vec<&str> = str.split(":").collect();
    let right: Vec<f32> = left[1].split(",")
        .into_iter()
        .map(|x| x.trim().parse::<f32>())
        .filter_map(Result::ok)
        .collect();

    let line_item = Item::new_item(
        left[0],
        right[0] as u32,
        right [1]
    ).await;

    line_item
   
}

fn table_format(items: Vec<Item>) -> Table {
    let mut totals_vec: Vec<f32> = vec![0.0;7];

    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(&[format::LinePosition::Top,
                  format::LinePosition::Bottom],
                format::LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build();

    table.set_format(format);
    table.set_titles(row!["#", "Name", "Amount", "(O)Price", "(O)Total", "(N)Price", "(N)Total", "Price +/-", "Total +/-"]);
    
    for (idx, itm) in items.iter().enumerate() { 
    //    update_totals(&mut totals, itm); 
        update_totals_vec(&mut totals_vec, itm);
        table.add_row(row![
                      idx, 
                      urlencoding::decode(itm.name.as_str()).expect("UTF-8"),
                      itm.amount, 
                      format!("${:.2}", itm.buy_price),
                      format!("${:.2}", itm.buy_total),
                      format!("${:.2}", itm.sell_price),
                      format!("${:.2}", itm.sell_total),
                      format!("${:.2}", itm.sell_price - itm.buy_price),
                      format!("${:.2}", itm.sell_total - itm.buy_total),
                    
        ]);
    }

    table.add_row(row![
                  "+",
                  '-'.to_string().repeat(4),
                  '-'.to_string().repeat(4),
                  '-'.to_string().repeat(4),
                  '-'.to_string().repeat(4),
                  '-'.to_string().repeat(4),
                  '-'.to_string().repeat(4),
                  '-'.to_string().repeat(4),
                  '-'.to_string().repeat(4),
    ]);

    table.add_row(row!["+", "Totals",
                  totals_vec[0],
                  format!("${:.2}", totals_vec[1]),
                  format!("${:.2}", totals_vec[2]),
                  format!("${:.2}", totals_vec[3]),
                  format!("${:.2}", totals_vec[4]),
                  format!("${:.2}", totals_vec[5]),
                  format!("${:.2}", totals_vec[6]),
    ]);

    table
}

fn update_totals_vec(vec: &mut Vec<f32>, item: &Item) {
    let par_vec: Vec<f32> = vec![item.amount as f32,
        item.buy_price, item.buy_total,
        item.sell_price, item.sell_total,
        item.sell_price - item.buy_price,
        item.sell_total - item.buy_total,
    ];

    for x in 0..par_vec.len() {
        if let Some(val) = vec.get_mut(x) {
            *val += par_vec[x];
        }
    }
}
