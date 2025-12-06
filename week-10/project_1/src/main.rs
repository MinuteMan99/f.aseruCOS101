struct Laptop {
    brand:String,
    unit_price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.unit_price * quantity
    }
}

fn main() {
    let hp = Laptop { brand: "HP".to_string(), unit_price: 650_000 };
    let ibm = Laptop { brand: "IBM".to_string(), unit_price: 755_000 };
    let toshiba = Laptop { brand: "Toshiba".to_string(), unit_price: 550_000 };
    let dell = Laptop { brand: "Dell".to_string(), unit_price: 850_000 };

    let qty = 3;

    let total = hp.total_cost(qty)
        + ibm.total_cost(qty)
        + toshiba.total_cost(qty)
        + dell.total_cost(qty);

    println!("Total cost for 3 units of each brand = {}", total);
}