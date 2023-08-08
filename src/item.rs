pub struct Item {
    pub name: String,
    pub amount: u32,
    pub buy_price: f32,
    pub buy_total: f32,
    pub sell_price: f32,
    pub sell_total: f32,
}

impl Item {
    pub async fn new_item(name: &str, amount: u32, buy_price: f32) -> Item {
        let res = Item::get_price(name).await;
        let mut price_string = String::new();

        match res {
            Ok(string) => 
                price_string
                    .push_str(string["median_price"]
                    .as_str()
                    .expect("String appending error")),
            Err(e) => eprintln!("Problem fetching website. More Information: {}", e),
        };
       
        // Convert price to f32
        let sell_price = price_string[1..].parse::<f32>()
                .expect("Error parsing string");

        Item {
            name: name.to_string(),
            amount,
            buy_price,
            buy_total: buy_price * amount as f32,
            sell_price,
            sell_total: sell_price * amount as f32,
        }
    }

    async fn get_price(name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("https://steamcommunity.com/market/priceoverview/?appid=730&market_hash_name={}&currency=1", name);
        let res = reqwest::get(url)
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(res)
    }
}
