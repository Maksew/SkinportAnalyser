use std::io;

fn calculate_profit() -> (f64, f64) {
    let mut input = String::new();

    println!("Entrez le prix d'achat du skin' :");
    io::stdin().read_line(&mut input).unwrap();
    let purchase_price = input.trim().parse::<f64>().unwrap();

    input.clear();

    println!("Entrez le prix de vente :");
    io::stdin().read_line(&mut input).unwrap();
    let sell_price = input.trim().parse::<f64>().unwrap();

    let fee = if sell_price < 1000.0 {
        sell_price * 0.12
    } else {
        sell_price * 0.06
    };

    let profit = sell_price - fee - purchase_price;

    (purchase_price, profit)
}

fn main() {
    let (purchase_price, profit) = calculate_profit();

    println!("Prix d'achat initial : {:.2}", purchase_price);
    println!("Profit de :{}", purchase_price + profit);
    println!("+ {:.2}", profit);
}
