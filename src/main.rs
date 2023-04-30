mod db;
mod endpoints;

// Any errors croping up on the next 3 lines can just be ignored
#[macro_use]
extern crate rocket;
extern crate lazy_static;

use rocket_db_pools::Database;
use simple_logger::SimpleLogger;

#[cfg(test)]
pub mod tests {
    use lazy_static::lazy_static;
    use rocket::local::blocking::Client;
    use std::sync::Mutex;

    lazy_static! {
        pub static ref CLIENT: Mutex<Client> =
            Mutex::new(Client::tracked(super::rocket()).expect("valid rocket instance"));
    }
}

#[cfg(not(tarpaulin_include))]
#[launch]
fn rocket() -> _ {
    // Use simple logger for logging purposes
    match SimpleLogger::new().init() {
        Err(e) => println!("Logger Error: {e}"),
        _ => (),
    }

    // Rocket HTTP server creation routine
    rocket::build()
        .mount(
            "/",
            routes![
                endpoints::index,
                endpoints::auth::auth_credentials,
                endpoints::auth::auth_session,
                endpoints::auth::remove_session,
                endpoints::auth::auth_security_questions,
                endpoints::account::account_reset_password,
                endpoints::notes::fetch_protocols,
                endpoints::notes::fetch_notes,
                endpoints::notes::add_note,
                endpoints::notes::remove_note,
                endpoints::notes::update_note,
                endpoints::events::fetch_events,
                endpoints::rotations::fetch_rotations,
                endpoints::account::fetch_account,
                endpoints::account::update_account,
            ],
        )
        .attach(db::SPS::init())
}
