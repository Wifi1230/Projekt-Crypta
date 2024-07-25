import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AccEntry {
  'username' : string,
  'password' : string,
  'email' : string,
}
export interface Comment {
  'username' : string,
  'text' : string,
  'likes' : number,
  'wpis_index' : bigint,
  'dislikes' : number,
}
export interface CryptoEntry { 'name' : string, 'shortcut' : string }
export interface CryptoProposal {
  'name' : string,
  'likes' : number,
  'proposer' : string,
  'dislikes' : number,
  'shortcut' : string,
}
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export interface WpisAll {
  'username' : string,
  'post_text' : string,
  'prediction' : string,
  'likes' : number,
  'selected_crypto' : string,
  'dislikes' : number,
}
export interface _SERVICE {
  'add_account' : ActorMethod<[AccEntry], Result>,
  'add_crypto' : ActorMethod<[CryptoEntry], undefined>,
  'delete_account' : ActorMethod<[string], Result>,
  'dislike_comment' : ActorMethod<[string, bigint, bigint], undefined>,
  'dislike_proposal' : ActorMethod<[string, bigint], undefined>,
  'dislike_wpis' : ActorMethod<[string, bigint], undefined>,
  'dodaj_komentarz' : ActorMethod<[bigint, Comment], undefined>,
  'dodaj_wpis' : ActorMethod<[WpisAll], undefined>,
  'edytuj_wpis' : ActorMethod<[bigint, string], undefined>,
  'get_all_accounts' : ActorMethod<[], Array<AccEntry>>,
  'get_all_cryptos' : ActorMethod<[], Array<CryptoEntry>>,
  'get_all_proposals' : ActorMethod<[], Array<CryptoProposal>>,
  'like_comment' : ActorMethod<[string, bigint, bigint], undefined>,
  'like_proposal' : ActorMethod<[string, bigint], undefined>,
  'like_wpis' : ActorMethod<[string, bigint], undefined>,
  'odczytaj_komentarze' : ActorMethod<[bigint], Array<Comment>>,
  'odczytaj_wpisy' : ActorMethod<[], Array<WpisAll>>,
  'propose_crypto' : ActorMethod<[CryptoProposal], undefined>,
  'user_has_disliked' : ActorMethod<[string, bigint], boolean>,
  'user_has_disliked_comment' : ActorMethod<[string, bigint, bigint], boolean>,
  'user_has_disliked_proposal' : ActorMethod<[string, bigint], boolean>,
  'user_has_liked' : ActorMethod<[string, bigint], boolean>,
  'user_has_liked_comment' : ActorMethod<[string, bigint, bigint], boolean>,
  'user_has_liked_proposal' : ActorMethod<[string, bigint], boolean>,
  'usun_komentarz' : ActorMethod<[bigint, bigint], undefined>,
  'usun_krypto' : ActorMethod<[bigint], undefined>,
  'usun_propozycje' : ActorMethod<[bigint], undefined>,
  'usun_wpis' : ActorMethod<[bigint], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
