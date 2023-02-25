import { blob, CanisterResult, nat, ok, Principal, Query, Update, int32, Opt, Oneway, ic, Variant, nat64, $query } from 'azle';

$query
export function hello_world(): string {
    return 'Hello world!';
}