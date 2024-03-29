mod web;
mod email;
mod database;

fn main() {
    let ip_response = web::get_ip_address();
    match ip_response {
        Ok(ip_addr) => {
            // if ip address has changed then run the web automation script to update dns
            // println!("{}", ip_addr);

            // create table if does not exist
            database::database_functions::create_ip_table();
            database::database_functions::insert_ip(&ip_addr);
            database::database_functions::get_all_ip_addresses();
        }
        Err(e) => {
            eprintln!("An error occurred trying to fetch your IP address {}", e);
        }
    }
}
