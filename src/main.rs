mod web;
mod email;
mod database;

fn main() {
    // create table if does not exist
    // database::database_functions::create_ip_table();

    // for testing
    // if let Err(e) = web::web_functions::web_automation() {
    //     eprintln!("Error: {}", e);
    //     return;
    // }

    // get the current public ip address
    let ip_addr = match web::web_functions::get_ip_address() {
        Ok(ip_addr) => ip_addr,
        Err(e) => {
            eprintln!("An error occurred trying to fetch your IP address {}", e);
            return;
        }
    };

    // insert or update the newest public ip address
    if let Err(e) = database::database_functions::insert_or_update(&ip_addr) {
        println!("Error inserting or updating IP address: {}", e);
        return;
    }

    // query ip address in table
    let previous_ip = match database::database_functions::get_latest_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Error querying previous IP address: {}", e);
            return;
        }
    };

    // if the two ip addresses do not match then run the web automation script to update dns
    if ip_addr != previous_ip {
        match web::web_functions::web_automation() {
            Ok(_) => println!("Web automation script ran successfully"),
            Err(e) => eprintln!("Error running web automation script: {}", e),
        }
    }
    else {
        println!("The two IP addresses match: {}", previous_ip);
    }
}
