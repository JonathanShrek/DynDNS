mod web;
mod email;
mod database;

fn main() {
    // create table if does not exist
    database::database_functions::create_ip_table();

    // get the current public ip address
    let ip_response = web::web_functions::get_ip_address();
    match ip_response {
        Ok(ip_addr) => {
            // insert or update the newest public ip address
            database::database_functions::insert_or_update(&ip_addr);

            // query ip address in table
            let previous_ip = database::database_functions::get_latest_ip();
            match previous_ip {
                Ok(ip) => {
                    // if the two ip addresses do not match then run the web automation script to update dns
                    if ip_addr != ip {
                        let response = web::web_functions::web_automation();
                        match response {
                            Ok(_) => {
                                println!("Web automation script ran successfully");
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                    else {
                        print!("The two IP addresses match: {}", ip);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("An error occurred trying to fetch your IP address {}", e);
        }
    }
}
