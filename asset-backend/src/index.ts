// TODO once the Bitcoin integration is live, add the methods and tests

import { blob, nat, ok, Principal, $query, $update, ic } from 'azle';
import {
    CanisterStatusArgs,
    management_canister
} from 'azle/canisters/management';
import {
    DefaultResult,
    ExecuteCreateCanisterResult,
    GetCanisterStatusResult,
} from './types';

type State = {
    created_canister_id: Principal;
};

let state: State = {
    created_canister_id: Principal.fromText('aaaaa-aa')
};

let allowedUser: Principal = Principal.fromText('7zdi6-6h2gk-g4j54-cigti-iiu4u-lj4vy-bewjf-oouoc-dnlck-fyfy5-aae');

$update;
export async function createCanister(): Promise<ExecuteCreateCanisterResult> {
    if (ic.caller() !== allowedUser) {
        return {
            err: 'Only allowed user can create canister'
        };
    } else {
        const create_canister_result_canister_result = await management_canister
            .create_canister({
                settings: null
            })
            .cycles(300_000_000_000n)
            .call();

        if (!ok(create_canister_result_canister_result)) {
            return {
                err: create_canister_result_canister_result.err
            };
        }

        const create_canister_result = create_canister_result_canister_result.ok;

        state.created_canister_id = create_canister_result.canister_id;

        return {
            ok: create_canister_result
        };
    }
}

$update;
export async function updateCanisterSettings(
    canister_id: Principal,
): Promise<DefaultResult> {
    if (ic.caller() !== allowedUser) {
        return {
            err: 'Only allowed user can update canister settings'
        };
    } else {
        const canister_result = await management_canister
            .update_settings({
                canister_id,
                settings: {
                    controllers: [ic.caller()],
                    compute_allocation: null,
                    memory_allocation: null,
                    freezing_threshold: null
                }
            })
            .call();

        if (!ok(canister_result)) {
            return {
                err: canister_result.err
            };
        }

        return {
            ok: true
        };
    }
}

$update;
export async function installAssetWasm(
    canister_id: Principal,
    wasm_module: blob
): Promise<DefaultResult> {
    if (ic.caller() !== allowedUser) {
        return {
            err: 'Only allowed user can install wasm'
        };
    } else {
        const canister_result = await management_canister
            .install_code({
                mode: {
                    install: null
                },
                canister_id,
                wasm_module,
                arg: Uint8Array.from([])
            })
            .cycles(100_000_000_000n)
            .call();

        if (!ok(canister_result)) {
            return {
                err: canister_result.err
            };
        }

        return {
            ok: true
        };
    }
}

$update;
export async function getCanisterStatus(
    args: CanisterStatusArgs
): Promise<GetCanisterStatusResult> {
    const canister_status_result_canister_result = await management_canister
        .canister_status({
            canister_id: args.canister_id
        })
        .call();

    if (canister_status_result_canister_result.ok === undefined) {
        return {
            err: canister_status_result_canister_result.err
        };
    }

    const canister_status_result = canister_status_result_canister_result.ok;

    return {
        ok: canister_status_result
    };
}

$query;
export function get_created_canister_id(): Principal {
    return state.created_canister_id;
}
