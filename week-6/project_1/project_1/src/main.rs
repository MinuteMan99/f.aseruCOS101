use std::io;

// Function to handle reading the user's choice
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    // Define the menu and prices using constants for clear reference
    const PRICE_P: i32 = 3200; // Poundo Yam/Edinkaiko Soup
    const PRICE_F: i32 = 3000; // Fried Rice & Chicken
    const PRICE_A: i32 = 2500; // Amala & Ewedu Soup
    const PRICE_E: i32 = 2000; // Eba & Egusi Soup
    const PRICE_W: i32 = 2500; // White Rice & Stew

    println!("=======================================");
    println!("     Welcome to the PAU Canteen!");
    println!("=======================================");
    println!("Here is our menu:");
    println!("P: Poundo Yam/Edinkaiko Soup - N{} (per plate)", PRICE_P);
    println!("F: Fried Rice & Chicken      - N{} (per plate)", PRICE_F);
    println!("A: Amala & Ewedu Soup        - N{} (per plate)", PRICE_A);
    println!("E: Eba & Egusi Soup          - N{} (per plate)", PRICE_E);
    println!("W: White Rice & Stew         - N{} (per plate)", PRICE_W);
    println!("=======================================");

    let mut total_price: i32 = 0; // The accumulator for the total cost
    let mut item_name = "";
    let mut item_price = 0;
    let mut quantity = 0;

    // --- 1. Get User's Meal Choice ---
    loop {
        println!("\nPlease enter the code for the meal you want (P, F, A, E, W):");
        let choice = read_input().to_uppercase(); // Convert to uppercase for easy matching

        if choice.len() != 1 {
             println!("Invalid input. Please enter only one code (P, F, A, E, W).");
             continue;
        }

        match choice.as_str() {
            "P" => { item_name = "Poundo Yam/Edinkaiko Soup"; item_price = PRICE_P; break; }
            "F" => { item_name = "Fried Rice & Chicken"; item_price = PRICE_F; break; }
            "A" => { item_name = "Amala & Ewedu Soup"; item_price = PRICE_A; break; }
            "E" => { item_name = "Eba & Egusi Soup"; item_price = PRICE_E; break; }
            "W" => { item_name = "White Rice & Stew"; item_price = PRICE_W; break; }
            _ => println!("Invalid choice. Please select P, F, A, E, or W."),
        }
    }

    // --- 2. Get Quantity ---
    loop {
        println!("\nHow many plates of {} would you like?", item_name);
        let quantity_input = read_input();
        
        match quantity_input.parse::<i32>() {
            Ok(q) if q > 0 => {
                quantity = q;
                break;
            }
            _ => println!("Invalid quantity. Please enter a positive number."),
        }
    }

    // --- 3. Calculate Total Price (using multiplication operator) ---
    // The `total_price` variable is calculated by multiplying the unit price and the quantity.
    total_price = item_price * quantity;
    
    let single_item_total = item_price * quantity;
    let single_item_price_display: f32 = item_price as f32 / 1000.0;
    let final_total_display: f32 = total_price as f32 / 1000.0;


    // --- 4. Display Receipt ---
    println!("\n=======================================");
    println!("           ORDER RECEIPT");
    println!("=======================================");
    println!("Item:     {}", item_name);
    println!("Price:    N{}k", single_item_price_display);
    println!("Quantity: {}", quantity);
    println!("---------------------------------------");
    println!("TOTAL:    N{}k", final_total_display);
    println!("=======================================");
    println!("Thank you for your order!");
}