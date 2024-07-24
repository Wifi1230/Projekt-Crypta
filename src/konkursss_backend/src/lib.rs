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

#[derive(Clone, CandidType, Deserialize, Default)]
struct Comment {
    text: String,
    username: String,
    likes: u32,
    dislikes: u32,
    wpis_index: usize,
}

// Struktura do przechowywania głosów użytkowników
#[derive(Clone, CandidType, Deserialize, Default)]
struct UserVotes {
    liked: HashSet<String>,
    disliked: HashSet<String>,
}

// Przechowywanie komentarzy dla każdego wpisu
thread_local! {
    static WPISY: RefCell<Vec<WpisAll>> = RefCell::default();
    static KRYPTO: RefCell<Vec<CryptoEntry>> = RefCell::new(Vec::new());
    static ACCOUNTS: RefCell<Vec<AccEntry>> = RefCell::new(Vec::new());
    static VOTES: RefCell<HashMap<usize, UserVotes>> = RefCell::new(HashMap::new());
    static COMMENT_VOTES: RefCell<HashMap<(usize, usize), UserVotes>> = RefCell::new(HashMap::new());
    static COMMENTS: RefCell<HashMap<usize, Vec<Comment>>> = RefCell::new(HashMap::new());
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
        if wpisy.borrow().len() > id_wpisu {
            wpisy.borrow_mut().remove(id_wpisu);
        }
    });
    COMMENTS.with(|comments| {
        comments.borrow_mut().remove(&id_wpisu);
    });
    VOTES.with(|votes| {
        votes.borrow_mut().remove(&id_wpisu);
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

// Dodawanie komentarza
#[ic_cdk::update]
fn dodaj_komentarz(wpis_index: usize, comment: Comment) {
    COMMENTS.with(|comments| {
        comments.borrow_mut().entry(wpis_index).or_default().push(comment);
    });
}

// Odczytywanie komentarzy dla danego wpisu
#[ic_cdk::query]
fn odczytaj_komentarze(wpis_index: usize) -> Vec<Comment> {
    COMMENTS.with(|comments| {
        comments.borrow().get(&wpis_index).cloned().unwrap_or_default()
    })
}

// Lubię to dla komentarza
#[ic_cdk::update]
fn like_comment(user_id: String, wpis_index: usize, comment_index: usize) {
    COMMENTS.with(|comments| {
        if let Some(comment_list) = comments.borrow_mut().get_mut(&wpis_index) {
            if let Some(comment) = comment_list.get_mut(comment_index) {
                let mut votes = COMMENT_VOTES.with(|v| v.borrow_mut().entry((wpis_index, comment_index)).or_default().clone());

                if votes.liked.contains(&user_id) {
                    // Anuluj "like"
                    votes.liked.remove(&user_id);
                    comment.likes -= 1;
                } else {
                    // Jeśli był "dislike", zamień na "like"
                    if votes.disliked.contains(&user_id) {
                        votes.disliked.remove(&user_id);
                        comment.dislikes -= 1;
                    }
                    votes.liked.insert(user_id.clone());
                    comment.likes += 1;
                }

                COMMENT_VOTES.with(|v| v.borrow_mut().insert((wpis_index, comment_index), votes));
            }
        }
    });
}

// Nie lubię dla komentarza
#[ic_cdk::update]
fn dislike_comment(user_id: String, wpis_index: usize, comment_index: usize) {
    COMMENTS.with(|comments| {
        if let Some(comment_list) = comments.borrow_mut().get_mut(&wpis_index) {
            if let Some(comment) = comment_list.get_mut(comment_index) {
                let mut votes = COMMENT_VOTES.with(|v| v.borrow_mut().entry((wpis_index, comment_index)).or_default().clone());

                if votes.disliked.contains(&user_id) {
                    // Anuluj "dislike"
                    votes.disliked.remove(&user_id);
                    comment.dislikes -= 1;
                } else {
                    // Jeśli był "like", zamień na "dislike"
                    if votes.liked.contains(&user_id) {
                        votes.liked.remove(&user_id);
                        comment.likes -= 1;
                    }
                    votes.disliked.insert(user_id.clone());
                    comment.dislikes += 1;
                }

                COMMENT_VOTES.with(|v| v.borrow_mut().insert((wpis_index, comment_index), votes));
            }
        }
    });
}

// Sprawdzanie, czy użytkownik polubił komentarz
#[ic_cdk::query]
fn user_has_liked_comment(user_id: String, wpis_index: usize, comment_index: usize) -> bool {
    COMMENT_VOTES.with(|v| {
        v.borrow().get(&(wpis_index, comment_index)).map_or(false, |votes| votes.liked.contains(&user_id))
    })
}

// Sprawdzanie, czy użytkownik nie polubił komentarza
#[ic_cdk::query]
fn user_has_disliked_comment(user_id: String, wpis_index: usize, comment_index: usize) -> bool {
    COMMENT_VOTES.with(|v| {
        v.borrow().get(&(wpis_index, comment_index)).map_or(false, |votes| votes.disliked.contains(&user_id))
    })
}