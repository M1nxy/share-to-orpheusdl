# Share to OrpheusDL

## WIP
This is currently a work in progress. Things like a tray icon are not added yet. There are currently no distributed binaries for the app and the extension is unpacked. **Things will break, you have been warned!**

### Temporary setup instructions
1. To load the extension goto `chrome://extensions` and load the `extension` folder as unpacked.

2. To run the app :
```sh
git clone https://github.com/M1nxy/share-to-orpheusdl.git
cd app
cargo run // to run globally you can also cargo install --path .
```

## App
> Prerequisite: You will need orpheusdl and appropriate modules downloaded wherever you run this

This component should run wherever you store your media and hosts a small http server to recieve new download tasks. They are added to a queue and proccessed in a seperate thread asynchronously. You can see the ouput from these tasks in your appdata folder under `orpheusdl-tray\logs`.

On first run it will create a config.toml file in the same directory. This will need to be updated to include your orpehusdl install path and a random token that you will need to enter into the browser extension. This token is insecure and should not be used as a replacement for a reverse proxy if exposing to the network.

## Extension
This component runs in the browser and forwards the current page to your OrpheusDL install when the extension icon is pressed. 

It has a few config options: 
- Automatically confirm downloads: This automatically adds tasks to queue and completely prevents the confirmation popup.
- Override Tray URI: This enables the alternative URI.
- Token: This should be the same as the token set in the `config.toml` for the App.


