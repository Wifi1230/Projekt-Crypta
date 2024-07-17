import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AccEntry {
  'username' : string,
  'password' : string,
  'email' : string,
}
export interface CryptoEntry {
  'icon' : string,
  'name' : string,
  'shortcut' : string,
}
export interface WpisAll {
  'username' : string,
  'post_text' : string,
  'prediction' : string,
  'likes' : number,
  'selected_crypto' : string,
  'dislikes' : number,
}
export interface _SERVICE {
  'add_account' : ActorMethod<[AccEntry], undefined>,
  'add_crypto' : ActorMethod<[CryptoEntry], undefined>,
  'dislike_wpis' : ActorMethod<[bigint], undefined>,
  'dodaj_wpis' : ActorMethod<[WpisAll], undefined>,
  'edytuj_wpis' : ActorMethod<[bigint, string], undefined>,
  'get_all_accounts' : ActorMethod<[], Array<AccEntry>>,
  'get_all_cryptos' : ActorMethod<[], Array<CryptoEntry>>,
  'like_wpis' : ActorMethod<[bigint], undefined>,
  'odczytaj_wpisy' : ActorMethod<[], Array<WpisAll>>,
  'usun_wpis' : ActorMethod<[bigint], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
