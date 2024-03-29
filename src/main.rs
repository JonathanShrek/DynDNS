mod email;
mod web;

fn main() {
    let ip_response = web::get_ip_address();
    match ip_response {
        Ok(ip_addr) => {
            // if ip address has changed then run the web automation script to update dns
            println!("{}", ip_addr);
        }
        Err(e) => {
            eprintln!("An error occurred trying to fetch your IP address {}", e);
        }
    }
}
