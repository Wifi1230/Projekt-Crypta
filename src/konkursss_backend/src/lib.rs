use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use candid::{CandidType, Deserialize};

#[derive(Clone, CandidType, Deserialize, Default)]
struct CryptoEntry {
    name: String,
    shortcut: String,
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

#[derive(Clone, CandidType, Deserialize, Default)]
struct CryptoProposal {
    name: String,
    shortcut: String,
    proposer: String,
    likes: u32,
    dislikes: u32,
}

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
    static COMMENT_VOTES: RefCell<HashMap<(usize, usize), UserVotes>> = RefCell::new(HashMap::new());
    static COMMENTS: RefCell<HashMap<usize, Vec<Comment>>> = RefCell::new(HashMap::new());
    static PROPOSALS: RefCell<Vec<CryptoProposal>> = RefCell::new(Vec::new());
    static PROPOSAL_VOTES: RefCell<HashMap<usize, UserVotes>> = RefCell::new(HashMap::new());
}


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
                votes.liked.remove(&user_id);
                wpis.likes -= 1;
            } else {
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
                votes.disliked.remove(&user_id);
                wpis.dislikes -= 1;
            } else {
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

#[ic_cdk::update]
fn usun_krypto(index: usize) {
    KRYPTO.with(|crypto| {
        if index < crypto.borrow().len() {
            crypto.borrow_mut().remove(index);
        }
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
#[ic_cdk::update]
fn delete_account(username: String) -> Result<(), String> {
    ACCOUNTS.with(|accounts| {
        let mut accounts = accounts.borrow_mut();
        if let Some(pos) = accounts.iter().position(|acc| acc.username == username) {
            accounts.remove(pos);
            
            WPISY.with(|wpisy| {
                let mut wpisy = wpisy.borrow_mut();
                wpisy.retain(|wpis| wpis.username != username);
            });
            
            COMMENTS.with(|comments| {
                let mut comments = comments.borrow_mut();
                for comment_list in comments.values_mut() {
                    comment_list.retain(|comment| comment.username != username);
                }
            });
            
            VOTES.with(|votes| {
                let mut votes = votes.borrow_mut();
                for (_, user_votes) in votes.iter_mut() {
                    user_votes.liked.remove(&username);
                    user_votes.disliked.remove(&username);
                }
            });
            
            COMMENT_VOTES.with(|comment_votes| {
                let mut comment_votes = comment_votes.borrow_mut();
                for (_, user_votes) in comment_votes.iter_mut() {
                    user_votes.liked.remove(&username);
                    user_votes.disliked.remove(&username);
                }
            });
            
            PROPOSAL_VOTES.with(|proposal_votes| {
                let mut proposal_votes = proposal_votes.borrow_mut();
                for (_, user_votes) in proposal_votes.iter_mut() {
                    user_votes.liked.remove(&username);
                    user_votes.disliked.remove(&username);
                }
            });
            PROPOSALS.with(|proposals| {
                let mut proposals = proposals.borrow_mut();
                proposals.retain(|proposal| proposal.proposer != username);
            });

            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    })
}

// Dodawanie komentarza
#[ic_cdk::update]
fn dodaj_komentarz(wpis_index: usize, comment: Comment) {
    COMMENTS.with(|comments| {
        comments.borrow_mut().entry(wpis_index).or_default().push(comment);
    });
}

// Usuwanie komentarza
#[ic_cdk::update]
fn usun_komentarz(wpis_index: usize, comment_index: usize) {
    COMMENTS.with(|comments| {
        if let Some(comment_list) = comments.borrow_mut().get_mut(&wpis_index) {
            if comment_index < comment_list.len() {
                comment_list.remove(comment_index);
                COMMENT_VOTES.with(|votes| {
                    votes.borrow_mut().remove(&(wpis_index, comment_index));
                    let keys_to_adjust: Vec<_> = votes
                        .borrow()
                        .keys()
                        .filter(|&&(wi, ci)| wi == wpis_index && ci > comment_index)
                        .cloned()
                        .collect();

                    for key in keys_to_adjust {
                        if let Some(entry) = votes.borrow_mut().remove(&key) {
                            votes.borrow_mut().insert((key.0, key.1 - 1), entry);
                        }
                    }
                });
            }
        }
    });
}
#[ic_cdk::query]
fn odczytaj_komentarze(wpis_index: usize) -> Vec<Comment> {
    COMMENTS.with(|comments| {
        comments.borrow().get(&wpis_index).cloned().unwrap_or_default()
    })
}
#[ic_cdk::update]
fn like_comment(user_id: String, wpis_index: usize, comment_index: usize) {
    COMMENTS.with(|comments| {
        if let Some(comment_list) = comments.borrow_mut().get_mut(&wpis_index) {
            if let Some(comment) = comment_list.get_mut(comment_index) {
                let mut votes = COMMENT_VOTES.with(|v| v.borrow_mut().entry((wpis_index, comment_index)).or_default().clone());

                if votes.liked.contains(&user_id) {
                    votes.liked.remove(&user_id);
                    comment.likes -= 1;
                } else {
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
#[ic_cdk::update]
fn dislike_comment(user_id: String, wpis_index: usize, comment_index: usize) {
    COMMENTS.with(|comments| {
        if let Some(comment_list) = comments.borrow_mut().get_mut(&wpis_index) {
            if let Some(comment) = comment_list.get_mut(comment_index) {
                let mut votes = COMMENT_VOTES.with(|v| v.borrow_mut().entry((wpis_index, comment_index)).or_default().clone());

                if votes.disliked.contains(&user_id) {
                    votes.disliked.remove(&user_id);
                    comment.dislikes -= 1;
                } else {
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
#[ic_cdk::query]
fn user_has_liked_comment(user_id: String, wpis_index: usize, comment_index: usize) -> bool {
    COMMENT_VOTES.with(|v| {
        v.borrow().get(&(wpis_index, comment_index)).map_or(false, |votes| votes.liked.contains(&user_id))
    })
}
#[ic_cdk::query]
fn user_has_disliked_comment(user_id: String, wpis_index: usize, comment_index: usize) -> bool {
    COMMENT_VOTES.with(|v| {
        v.borrow().get(&(wpis_index, comment_index)).map_or(false, |votes| votes.disliked.contains(&user_id))
    })
}
#[ic_cdk::update]
fn propose_crypto(proposal: CryptoProposal) {
    PROPOSALS.with(|proposals| {
        proposals.borrow_mut().push(proposal);
    });
}
#[ic_cdk::update]
fn usun_propozycje(proposal_index: usize) {
    PROPOSALS.with(|proposals| {
        if proposal_index < proposals.borrow().len() {
            proposals.borrow_mut().remove(proposal_index);
            PROPOSAL_VOTES.with(|votes| {
                votes.borrow_mut().remove(&proposal_index);
                let mut keys_to_adjust = Vec::new();
                for (key, _) in votes.borrow().iter() {
                    if *key > proposal_index {
                        keys_to_adjust.push(*key);
                    }
                }
                for key in keys_to_adjust {
                    if let Some(entry) = votes.borrow_mut().remove(&key) {
                        votes.borrow_mut().insert(key - 1, entry);
                    }
                }
            });
        }
    });
}
#[ic_cdk::query]
fn get_all_proposals() -> Vec<CryptoProposal> {
    PROPOSALS.with(|proposals| {
        proposals.borrow().clone()
    })
}

#[ic_cdk::update]
fn like_proposal(user_id: String, proposal_index: usize) {
    PROPOSALS.with(|proposals| {
        let mut proposals_mut = proposals.borrow_mut();
        if let Some(proposal) = proposals_mut.get_mut(proposal_index) {
            let mut votes = PROPOSAL_VOTES.with(|v| v.borrow_mut().entry(proposal_index).or_default().clone());

            if votes.liked.contains(&user_id) {
                votes.liked.remove(&user_id);
                proposal.likes -= 1;
            } else {
                if votes.disliked.contains(&user_id) {
                    votes.disliked.remove(&user_id);
                    proposal.dislikes -= 1;
                }
                votes.liked.insert(user_id.clone());
                proposal.likes += 1;
            }

            PROPOSAL_VOTES.with(|v| v.borrow_mut().insert(proposal_index, votes));

            if proposal.likes >= 5 {
                add_crypto(CryptoEntry {
                    name: proposal.name.clone(),
                    shortcut: proposal.shortcut.clone(),
                });
                proposals_mut.remove(proposal_index);
            }
        }
    });
}

#[ic_cdk::update]
fn dislike_proposal(user_id: String, proposal_index: usize) {
    PROPOSALS.with(|proposals| {
        let mut proposals_mut = proposals.borrow_mut();
        if let Some(proposal) = proposals_mut.get_mut(proposal_index) {
            let mut votes = PROPOSAL_VOTES.with(|v| v.borrow_mut().entry(proposal_index).or_default().clone());

            if votes.disliked.contains(&user_id) {
                votes.disliked.remove(&user_id);
                proposal.dislikes -= 1;
            } else {
                if votes.liked.contains(&user_id) {
                    votes.liked.remove(&user_id);
                    proposal.likes -= 1;
                }
                votes.disliked.insert(user_id.clone());
                proposal.dislikes += 1;
            }

            PROPOSAL_VOTES.with(|v| v.borrow_mut().insert(proposal_index, votes));
            if proposal.dislikes >= 5 {
                proposals_mut.remove(proposal_index);
            }
        }
    });
}

#[ic_cdk::query]
fn user_has_liked_proposal(user_id: String, proposal_index: usize) -> bool {
    PROPOSAL_VOTES.with(|v| {
        v.borrow().get(&proposal_index).map_or(false, |votes| votes.liked.contains(&user_id))
    })
}

#[ic_cdk::query]
fn user_has_disliked_proposal(user_id: String, proposal_index: usize) -> bool {
    PROPOSAL_VOTES.with(|v| {
        v.borrow().get(&proposal_index).map_or(false, |votes| votes.disliked.contains(&user_id))
    })
}

