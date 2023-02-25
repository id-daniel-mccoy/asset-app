## The Asset App
### Version 0.3.1

### Easy Setup:

We tried to make this as easy as possible through the use of intuitve scripts. You should be able to setup the frontend and backend with one go using this command:
```
npm run quick-setup
```

Note: As of now you will be required to enter your dfx identity password twice during this sequence if you have set one. Also, it is fairly normal for the setup process to take quite a few minutes on the first run. Feel free to sit back and relax, let the program do the work. You will see a confirmation message upon successful completion.

### Manual Setup:

If for some reason the easy setup is not working for you, here are the manual steps to take to make sure you are setup properly.

#### Install Dependencies

```
npm run install-deps
```

#### Setup Frontend:

```
cd asset-frontend
dfx start --clean --background
dfx deploy
dfx stop && cd ../
```

#### Setup Backend:

```
cd asset-backend
dfx start --clean --background
dfx canister create --all
npx azle backend
dfx stop && cd ../
```

You should now be ready to continue!

...