# Chaz

A webassembly game made with Bevy for the first Bevy game jam.

## Prerequisites

```bash
rustup target install wasm32-unknown-unknown +nightly
cargo install wasm-server-runner
cargo install wasm-bindgen-cli
```

To manage itch.io uploads from the command line, install [butler](https://itch.io/docs/butler/installing.html).

## Run locally

```bash
cargo run
```

## Deploy
Does not work for me on Firefox for some reason, but works on Chrome.

### Build
```bash
cargo build --release
wasm-bindgen --out-dir out/pkg --target web target/wasm32-unknown-unknown/release/chaz.wasm
rsync -a assets/ out/assets/
```

### Deploy locally
Build, then run this:
```bash
cd out
python3 -m http.server <port>
```

### Deploy to [itch.io](https://itch.io)
Build, then run this:
```bash
butler login # follow the instructions
zip -r chaz.zip out
butler push chaz.zip luizchagasjardim/chaz:html
```
If this is the first time uploading to the html channel,
you need to go to the game page on itch.io and click on _Edit game_,
then set it to playable in the browser.

# TODO

* Go through the code and fix everything marked with a TODO
* Fix compilation targeting desktop
* Add scripts or cargo make commands to do stuff
* Add sound effects
