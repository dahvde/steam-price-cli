use std::fs::File;
use csv::Writer;
use tabled::{builder::Builder, settings::{Style, style::{RawStyle, BorderText}}};

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub name_uri: String,
    pub amount: u32,
    pub buy_price: f32,
    pub buy_total: f32,
    pub sell_price: f32,
    pub sell_total: f32,
}

pub struct ItemCollection {
    pub items: Vec<Item>,
    pub sell_price: f32,
    pub sell_total: f32,
    pub buy_price: f32,
    pub buy_total: f32,
    pub profit_price: f32,
    pub profit_total: f32,
    pub amount: u32,
}

impl ItemCollection {
    pub fn init(vec_items: &Vec<Item>) -> ItemCollection {
        let mut items = ItemCollection{
            sell_price: 0f32,
            items: vec_items.clone(),
            sell_total: 0f32,
            buy_price: 0f32,
            buy_total: 0f32,
            profit_price: 0f32,
            profit_total: 0f32,
            amount: 0u32,
        };

        for x in vec_items {
            items.sell_price += x.sell_price;
            items.sell_total += x.sell_total;
            items.buy_price += x.buy_price;
            items.buy_total += x.buy_total;
            items.amount += x.amount;
        }

        items.profit_price = items.sell_price - items.buy_price;
        items.profit_total = items.sell_total - items.buy_total;

        items
    }


    pub fn table_print(&self) {
        let mut builder = Builder::default();
        builder
            .set_header([
                "#",
                "Name",
                "Amnt",
                "(O)P",
                "(O)T",
                "(N)P",
                "(N)T",
                "P +/-",
                "T +/-"
            ]
        );

        for (idx, itm) in self.items.iter().enumerate() {
            builder.push_record(
                [
                    idx.to_string(),
                    itm.name.to_owned(),
                    itm.amount.to_string(),
                    format!("${:.2}", itm.buy_price),
                    format!("${:.2}", itm.buy_total),
                    format!("${:.2}", itm.sell_price),
                    format!("${:.2}", itm.sell_total),
                    format!("${:.2}", itm.sell_price - itm.buy_price),
                    format!("${:.2}", itm.sell_total - itm.buy_total),
                ]
            );
        }

        builder.push_record(
            [
                "+".to_string(),
                "".to_string(),
                self.amount.to_string(),
                format!("${:.2}", self.buy_price),
                format!("${:.2}", self.buy_total),
                format!("${:.2}", self.sell_price),
                format!("${:.2}", self.sell_total),
                format!("${:.2}", self.profit_price),
                format!("${:.2}", self.profit_total),
            ]
        );

        let mut style = RawStyle::from(Style::rounded());
        style.insert_horizontal(self.items.len() + 1, Style::modern().get_horizontal());

        let mut table = builder.build();
        table
            .with(style)
            .with(BorderText::new("Totals").horizontal(self.items.len() + 1));

        println!("{}", table);
    }
   
    /// TODO: Implement csv_writer
    pub fn _to_csv(&self, _writer: Writer<File>) {
        todo!();
    }
}

impl Item {
    pub async fn init(str: &str) -> Result<Item, String> {
        let (name, amount, buy_price) = Item::extract_values(str)?;

        let res = Item::get_price(name).await;
        let mut price_string = String::new();

        match res {
            Ok(string) => price_string.push_str(
                string["median_price"]
                    .as_str()
                    .expect("String appending error"),
            ),
            Err(e) => eprintln!("Problem fetching website. More Information: {}", e),
        };

        let sell_price = price_string[1..]
            .parse::<f32>()
            .expect("Error parsing string");

        Ok(Item {
            name: urlencoding::decode(name).expect("").to_string(),
            name_uri: name.to_string(),
            amount,
            buy_price,
            buy_total: buy_price * amount as f32,
            sell_price,
            sell_total: sell_price * amount as f32,
        })
    }

    fn extract_values(str: &str) -> Result<(&str, u32, f32), String> {
        let left: Vec<&str> = str.split(":").collect();
        let right: Vec<f32> = left[1]
            .split(",")
            .into_iter()
            .map(|x| x.trim().parse::<f32>())
            .filter_map(Result::ok)
            .collect();

        if right.len() != 2 {
            return Err(String::from("Amount or Buy price not specified"));
        }

        let name = left[0];
        let amount = right[0] as u32;
        let buy_price = right[1];
        Ok((name, amount, buy_price))
    }

    async fn get_price(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("https://steamcommunity.com/market/priceoverview/?appid=730&market_hash_name={}&currency=1", name);
        let res = reqwest::get(url).await?.json::<serde_json::Value>().await?;

        Ok(res)
    }
}
