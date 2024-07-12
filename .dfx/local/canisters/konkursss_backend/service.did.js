export const idlFactory = ({ IDL }) => {
  const AccEntry = IDL.Record({
    'username' : IDL.Text,
    'password' : IDL.Text,
    'email' : IDL.Text,
  });
  const CryptoEntry = IDL.Record({
    'icon' : IDL.Text,
    'name' : IDL.Text,
    'shortcut' : IDL.Text,
  });
  const WpisAll = IDL.Record({
    'post_text' : IDL.Text,
    'prediction' : IDL.Text,
    'likes' : IDL.Nat32,
    'selected_crypto' : IDL.Text,
    'dislikes' : IDL.Nat32,
  });
  return IDL.Service({
    'add_account' : IDL.Func([AccEntry], [], []),
    'add_crypto' : IDL.Func([CryptoEntry], [], []),
    'dislike_wpis' : IDL.Func([IDL.Nat64], [], []),
    'dodaj_wpis' : IDL.Func([WpisAll], [], []),
    'edytuj_wpis' : IDL.Func([IDL.Nat64, IDL.Text], [], []),
    'get_all_accounts' : IDL.Func([], [IDL.Vec(AccEntry)], ['query']),
    'get_all_cryptos' : IDL.Func([], [IDL.Vec(CryptoEntry)], ['query']),
    'like_wpis' : IDL.Func([IDL.Nat64], [], []),
    'odczytaj_wpisy' : IDL.Func([], [IDL.Vec(WpisAll)], ['query']),
    'usun_wpis' : IDL.Func([IDL.Nat64], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
