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
export interface _SERVICE {
  'add_account' : ActorMethod<[AccEntry], undefined>,
  'add_crypto' : ActorMethod<[CryptoEntry], undefined>,
  'dodaj_wpis' : ActorMethod<[string], undefined>,
  'edytuj_wpis' : ActorMethod<[bigint, string], undefined>,
  'get_all_accounts' : ActorMethod<[], Array<AccEntry>>,
  'get_all_cryptos' : ActorMethod<[], Array<CryptoEntry>>,
  'odczytaj_wpisy' : ActorMethod<[], Array<string>>,
  'usun_wpis' : ActorMethod<[bigint], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
