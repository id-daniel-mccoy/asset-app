## The Asset App Backend
### Version 0.2

### Getting Started:

#### Dependencies:

To compile, use, or deploy this project you will need dfx, rust/cargo, and node/npm.

#### To Build:

```
npm install
dfx canister create --all
dfx build
```
Note: The default canister name is 'backend'. You can change that by modifying the dfx.json file's canister section. If you modify it and plan to deploy on the mainnet make sure you also edit the canister_ids.json file otherwise it will create another new canister for you.

#### To Install Wasm To Canister:

The default dfx deploy command will not work due to the default wasm payload being over 2MB. To avoid the error and install the wasm directly use:

```
dfx canister install backend --wasm .azle/backend/backend.wasm.gz --mode reinstall -y
```

Disclaimer: Any method exposed by this canister is experimental and is subject to sudden changes, errors, and inconsistent operation, and potentially may not be permissioned. Use at your own risk.