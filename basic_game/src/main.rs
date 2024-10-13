use std::io;
use rand::Rng;

fn main() {
    let mut nbr_day = String::new();
    println!("Welcome to the game =)");
    println!("Enter number of days (number of simulation days) : ");
    io::stdin().read_line(&mut nbr_day).expect("In error");
    let nbr_day: u32 = nbr_day.trim().parse().expect("Please enter a valid number!");
    
    //Create a very very very basic handmade order book with a seller and buyer vector.
    let mut data_buy: Vec<Vec<u8>> = Vec::new();
    let mut rand_buy = rand::thread_rng();
    for i in (5..=9).rev() {
        let nbr_rand_buy = rand_buy.gen_range(0..=10);
        data_buy.push(vec![i,nbr_rand_buy ]);
    }
    let mut data_sell: Vec<Vec<u8>> = Vec::new();
    let mut rand_sell = rand::thread_rng();
    for i in 10..=14 {
        let nbr_rand_sell = rand_sell.gen_range(0..=10);
        data_sell.push(vec![i,nbr_rand_sell ]);
    }

    // Enters a days loop and decrements the number of random quantities generated.
    let mut rng = rand::thread_rng();
    for i in 1..=nbr_day {
        let mut nbr_rand = rng.gen_range(0..=10);
        let trend_choice = rng.gen_range(0..=1);
        println!("====================");
        println!("DAY {i}");
        match trend_choice {
            0 => {
                println!("Selling trends -- {nbr_rand}");
                let mut i = 0;
                while i < data_buy.len(){
                    if nbr_rand >= data_buy[i][1] {
                        nbr_rand -= data_buy[i][1];
                        data_buy[i][1] = 0;
                        if nbr_rand == 0 {
                            break;
                        }
                    } else {
                        data_buy[i][1] -= nbr_rand;
                        nbr_rand = 0;
                        break;
                    }
                    i += 1;
                }
                if nbr_rand != 0 {
                    println!("Sellers win!");
                    break;
                }
                for elements in &data_buy {
                    println!("Price {} : Quantity {}", elements[0], elements[1]);
                }
            },
            1 => {
                println!("Buying trends ++ {nbr_rand}");
                let mut i = 0;
                while i < data_sell.len(){
                    if nbr_rand >= data_sell[i][1] {
                        nbr_rand -= data_sell[i][1];
                        data_sell[i][1] = 0;
                        if nbr_rand == 0 {
                            break;
                        }
                    } else {
                        data_sell[i][1] -= nbr_rand;
                        nbr_rand = 0;
                        break;
                    }
                    i += 1;
                }
                if nbr_rand != 0 {
                    println!("Buyers win!");
                    break;
                }
                for elements in &data_sell {
                    println!("Price {} : Quantity {}", elements[0], elements[1]);
                }
            },
            _ => println!("Error"),
        }        
    }
}
