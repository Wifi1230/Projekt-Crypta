export const idlFactory = ({ IDL }) => {
  const AccEntry = IDL.Record({
    'username' : IDL.Text,
    'password' : IDL.Text,
    'email' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const CryptoEntry = IDL.Record({ 'name' : IDL.Text, 'shortcut' : IDL.Text });
  const Comment = IDL.Record({
    'username' : IDL.Text,
    'text' : IDL.Text,
    'likes' : IDL.Nat32,
    'wpis_index' : IDL.Nat64,
    'dislikes' : IDL.Nat32,
  });
  const WpisAll = IDL.Record({
    'username' : IDL.Text,
    'post_text' : IDL.Text,
    'prediction' : IDL.Text,
    'likes' : IDL.Nat32,
    'selected_crypto' : IDL.Text,
    'dislikes' : IDL.Nat32,
  });
  const CryptoProposal = IDL.Record({
    'name' : IDL.Text,
    'likes' : IDL.Nat32,
    'proposer' : IDL.Text,
    'dislikes' : IDL.Nat32,
    'shortcut' : IDL.Text,
  });
  return IDL.Service({
    'add_account' : IDL.Func([AccEntry], [Result], []),
    'add_crypto' : IDL.Func([CryptoEntry], [], []),
    'delete_account' : IDL.Func([IDL.Text], [Result], []),
    'dislike_comment' : IDL.Func([IDL.Text, IDL.Nat64, IDL.Nat64], [], []),
    'dislike_proposal' : IDL.Func([IDL.Text, IDL.Nat64], [], []),
    'dislike_wpis' : IDL.Func([IDL.Text, IDL.Nat64], [], []),
    'dodaj_komentarz' : IDL.Func([IDL.Nat64, Comment], [], []),
    'dodaj_wpis' : IDL.Func([WpisAll], [], []),
    'edytuj_wpis' : IDL.Func([IDL.Nat64, IDL.Text], [], []),
    'get_all_accounts' : IDL.Func([], [IDL.Vec(AccEntry)], ['query']),
    'get_all_cryptos' : IDL.Func([], [IDL.Vec(CryptoEntry)], ['query']),
    'get_all_proposals' : IDL.Func([], [IDL.Vec(CryptoProposal)], ['query']),
    'like_comment' : IDL.Func([IDL.Text, IDL.Nat64, IDL.Nat64], [], []),
    'like_proposal' : IDL.Func([IDL.Text, IDL.Nat64], [], []),
    'like_wpis' : IDL.Func([IDL.Text, IDL.Nat64], [], []),
    'odczytaj_komentarze' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(Comment)],
        ['query'],
      ),
    'odczytaj_wpisy' : IDL.Func([], [IDL.Vec(WpisAll)], ['query']),
    'propose_crypto' : IDL.Func([CryptoProposal], [], []),
    'user_has_disliked' : IDL.Func(
        [IDL.Text, IDL.Nat64],
        [IDL.Bool],
        ['query'],
      ),
    'user_has_disliked_comment' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Nat64],
        [IDL.Bool],
        ['query'],
      ),
    'user_has_disliked_proposal' : IDL.Func(
        [IDL.Text, IDL.Nat64],
        [IDL.Bool],
        ['query'],
      ),
    'user_has_liked' : IDL.Func([IDL.Text, IDL.Nat64], [IDL.Bool], ['query']),
    'user_has_liked_comment' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Nat64],
        [IDL.Bool],
        ['query'],
      ),
    'user_has_liked_proposal' : IDL.Func(
        [IDL.Text, IDL.Nat64],
        [IDL.Bool],
        ['query'],
      ),
    'usun_komentarz' : IDL.Func([IDL.Nat64, IDL.Nat64], [], []),
    'usun_krypto' : IDL.Func([IDL.Nat64], [], []),
    'usun_propozycje' : IDL.Func([IDL.Nat64], [], []),
    'usun_wpis' : IDL.Func([IDL.Nat64], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
