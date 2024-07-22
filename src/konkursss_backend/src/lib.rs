use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use candid::{CandidType, Deserialize};

// Struktury danych
#[derive(Clone, CandidType, Deserialize, Default)]
struct CryptoEntry {
    name: String,
    shortcut: String,
    icon: String,
}

#[derive(Clone, CandidType, Deserialize, Default)]
struct AccEntry {
    username: String,
    email: String,
    password: String,
}

#[derive(Clone, CandidType, Deserialize, Default)]
struct WpisAll {
    post_text: String,
    selected_crypto: String,
    prediction: String,
    likes: u32,
    dislikes: u32,
    username: String,
}

// Struktura do przechowywania głosów użytkowników
#[derive(Clone, CandidType, Deserialize, Default)]
struct UserVotes {
    liked: HashSet<String>,
    disliked: HashSet<String>,
}

thread_local! {
    static WPISY: RefCell<Vec<WpisAll>> = RefCell::default();
    static KRYPTO: RefCell<Vec<CryptoEntry>> = RefCell::new(Vec::new());
    static ACCOUNTS: RefCell<Vec<AccEntry>> = RefCell::new(Vec::new());
    static VOTES: RefCell<HashMap<usize, UserVotes>> = RefCell::new(HashMap::new());
}

// Funkcje aktualizacyjne i zapytań

#[ic_cdk::update]
fn dodaj_wpis(entry: WpisAll) {
    WPISY.with(|wpisy| {
        wpisy.borrow_mut().push(entry);
    });
}

#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<WpisAll> {
    WPISY.with(|wpisy| {
        wpisy.borrow().clone()
    })
}

#[ic_cdk::update]
fn usun_wpis(id_wpisu: usize) {
    WPISY.with(|wpisy| {
        wpisy.borrow_mut().remove(id_wpisu);
    });
}

#[ic_cdk::update]
fn like_wpis(user_id: String, id_wpisu: usize) {
    WPISY.with(|wpisy| {
        if let Some(wpis) = wpisy.borrow_mut().get_mut(id_wpisu) {
            let mut votes = VOTES.with(|v| v.borrow_mut().entry(id_wpisu).or_default().clone());

            if votes.liked.contains(&user_id) {
                // Anuluj "like"
                votes.liked.remove(&user_id);
                wpis.likes -= 1;
            } else {
                // Jeśli był "dislike", zamień na "like"
                if votes.disliked.contains(&user_id) {
                    votes.disliked.remove(&user_id);
                    wpis.dislikes -= 1;
                }
                votes.liked.insert(user_id.clone());
                wpis.likes += 1;
            }

            VOTES.with(|v| v.borrow_mut().insert(id_wpisu, votes));
        }
    });
}

#[ic_cdk::update]
fn dislike_wpis(user_id: String, id_wpisu: usize) {
    WPISY.with(|wpisy| {
        if let Some(wpis) = wpisy.borrow_mut().get_mut(id_wpisu) {
            let mut votes = VOTES.with(|v| v.borrow_mut().entry(id_wpisu).or_default().clone());

            if votes.disliked.contains(&user_id) {
                // Anuluj "dislike"
                votes.disliked.remove(&user_id);
                wpis.dislikes -= 1;
            } else {
                // Jeśli był "like", zamień na "dislike"
                if votes.liked.contains(&user_id) {
                    votes.liked.remove(&user_id);
                    wpis.likes -= 1;
                }
                votes.disliked.insert(user_id.clone());
                wpis.dislikes += 1;
            }

            VOTES.with(|v| v.borrow_mut().insert(id_wpisu, votes));
        }
    });
}

#[ic_cdk::query]
fn user_has_liked(user_id: String, id_wpisu: usize) -> bool {
    VOTES.with(|v| {
        v.borrow().get(&id_wpisu).map_or(false, |votes| votes.liked.contains(&user_id))
    })
}

#[ic_cdk::query]
fn user_has_disliked(user_id: String, id_wpisu: usize) -> bool {
    VOTES.with(|v| {
        v.borrow().get(&id_wpisu).map_or(false, |votes| votes.disliked.contains(&user_id))
    })
}

#[ic_cdk::update]
fn add_crypto(entry: CryptoEntry) {
    KRYPTO.with(|crypto| {
        crypto.borrow_mut().push(entry);
    });
}

#[ic_cdk::query]
fn get_all_cryptos() -> Vec<CryptoEntry> {
    KRYPTO.with(|crypto| {
        crypto.borrow().clone()
    })
}

#[ic_cdk::update]
fn add_account(entry: AccEntry) -> Result<(), String> {
    ACCOUNTS.with(|accounts| {
        let mut accounts = accounts.borrow_mut();
        if accounts.iter().any(|acc| acc.username == entry.username) {
            return Err("Username already taken".to_string());
        }
        if accounts.iter().any(|acc| acc.email == entry.email) {
            return Err("Email already registered".to_string());
        }
        accounts.push(entry);
        Ok(())
    })
}

#[ic_cdk::query]
fn get_all_accounts() -> Vec<AccEntry> {
    ACCOUNTS.with(|accounts| {
        accounts.borrow().clone()
    })
}
