use std::cell::RefCell;
use candid::{CandidType, Deserialize};
#[derive(Clone, CandidType, Deserialize)]
struct CryptoEntry {
    name: String,
    shortcut: String,
    icon: String,
}

thread_local! {// Define a thread-local storage (TLS) variable WPISY, which holds a RefCell containing a Vec of Strings.
    static WPISY: RefCell<Vec<String>> = RefCell::default();

}
thread_local! {
    static KRYPTO: RefCell<Vec<CryptoEntry>> = RefCell::new(vec![
        CryptoEntry {
            name: String::from("Bitcoin"),
            shortcut: String::from("BTC"),
            icon: String::from("btc.png"),
        },
        CryptoEntry {
            name: String::from("Ethereum"),
            shortcut: String::from("ETH"),
            icon: String::from("eth.png"),
        },
    ]);
}
#[ic_cdk::update]
fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy| {// Access the thread-local WPISY variable.
        wpisy.borrow_mut().push(wpis)// Borrow a mutable reference to the Vec inside WPISY and push the new entry (wpis) into it.
    });
}

#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<String> {
    WPISY.with(|wpisy| {// Access the thread-local WPISY variable.
    wpisy.borrow().clone()// Borrow an immutable reference to the Vec inside WPISY and clone it to return.
    })
}

#[ic_cdk::update]
fn usun_wpis(id_wpisu: usize) {// Access the thread-local WPISY variable.
    WPISY.with(|wpisy| {// Borrow a mutable reference to the Vec inside WPISY and remove the entry at the specified index (id_wpisu).
    wpisy.borrow_mut().remove(id_wpisu)
    });
}

#[ic_cdk::update]
fn edytuj_wpis(id_wpisu: usize, nowy_wpis: String){
    WPISY.with(|wpisy|{// Access the WPISY variable, which is assumed to be a thread-local storage containing the entries.
        let mut binding=wpisy.borrow_mut(); // Borrow the mutable reference to the entries (wpisy).
        let wpis = binding.get_mut(id_wpisu);// Get a mutable reference to the entry at the given index (id_wpisu).
        let stary_wpis=wpis.unwrap();// Unwrap the Option to get the old entry, assuming the entry exists at the given index.
        *stary_wpis = nowy_wpis;// Replace the old entry with the new entry (nowy_wpis).
    });
}

#[ic_cdk::update]
fn add_crypto(entry:CryptoEntry) {
    KRYPTO.with(|crypto| {
        crypto.borrow_mut().push(entry);
    });
}
#[ic_cdk::query]
fn get_all_cryptos() -> Vec<CryptoEntry> {// Function to get all Cryptos from the thread-local storage
    KRYPTO.with(|crypto| {
        crypto.borrow().clone() // This clones the vector for returning
    })
}   
