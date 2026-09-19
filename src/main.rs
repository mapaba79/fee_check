fn main() {
    let resp = match reqwest::blocking::get(
        "https://mempool.space/api/v1/fees/recommended",
    ) {
        Ok(r) => r,
        Err(e) => {
            println!("Request error: {}", e);
            return;
        }
    };

    let json: serde_json::Value = match resp.json() {
        Ok(j) => j,
        Err(e) => {
            println!("erro ao parsear json: {}", e);
            return;
        }
    };

    println!("fastest: {}", json["fastestFee"]);
    println!("hour: {}", json["hourFee"]);
    println!("economy: {}", json["economyFee"]);
}
