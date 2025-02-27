use std::sync::Mutex;
use crate::promocode::PromoCode;
use std::mem;

// This file contains promocode functions implementation that need to call
// `SINGLETON.lock().unwrap();` to manage concurrent acces to database.
// It is a remember of using this call to use SINGLETON.
// This way lets to optimize performance with less code to maintain good usage of mutex.

/// Kind of singleton pattern without init needed for best performance.
static SINGLETON: Mutex<Vec<PromoCode>> = Mutex::new(vec![]);

impl PromoCode {

    /// Save a promocode in database.
    /// TODO BONUS : Save draft and recover it later.
    pub fn database_push(promocode: PromoCode) {
        let mut database = SINGLETON.lock().unwrap();

        // Lets to see that promocode is added for the test
        println!("### database_push({:#?})", promocode); 
        let size = mem::size_of_val(&promocode);
        if database.try_reserve(size).is_ok() {
            database.push(promocode);
        } else {
            eprintln!("DATABASE_PUSH_OUT_OF_MEMORY"); // out of memory
        }
    }

    /// Search if promo code with specified name exists in database.
    pub fn database_promocode_name_already_exists(promocode_name: &str) -> bool {
        let database = SINGLETON.lock().unwrap();
        if database.iter().any(|n| n.name == promocode_name) {
            return true;
        }
        false
    }

    /// Return a promocode with specified name if one exists.
    pub fn database_get(promocode_name: &str) -> Option<PromoCode> {
        let database = SINGLETON.lock().unwrap();

        for n in database.iter() {
            if n.name == promocode_name {
                match serde_json::to_string(n) {
                    Ok(promo_serialize) => {
                        // Serialize and deserialize in order to not borrow it.
                        // Clone and Copy not available because of the current json matching model used.
                        match serde_json::from_str::<PromoCode>(promo_serialize.as_str()) {
                            Ok(promocode) => { return Some(promocode) }
                            _ => { return None }
                        }
                    },
                    _ => { return None },
                }
            }
        }
        None
    }

}
