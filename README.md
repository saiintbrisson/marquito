# marquito e luizito

This project was conceived during TerraMagna's onboarding program.

`marquito` is a simple, hand-written, file storage that runs over HTTP written in Rust to store files (we do **not** support HTTPS nor do we plan to do so). 

`luizito` is a CLI client tool that sends and consumes files to/from `marquito`.

## Installation and Usage

Clone the repository and install `marquito` and `luizito` to `~/.cargo/bin`:

```sh
git clone https://github.com/saiintbrisson/marquito
cd marquito
cargo install --path ./marquito
cargo install --path ./luizito
```

Running the server is as easy as running `marquito` on the terminal:
```sh
marquito
INFO marquito::server: server is running addr=127.0.0.1:23400
```

`MARQUITO_ADDRESS` changes the server address (defaults to `127.0.0.1:23400`)  
`MARQUITO_DIRECTORY` changes the storage directory (defaults to `./files/`)

Now, with `luizito` installed, you can `send` and `get` files, here are some examples:

```sh
luizito send a.txt b.toml          # stores local files a.txt and b.toml
luizito get a.txt b.toml           # fetches stored files and displays them
luizito get --save a.txt b.toml    # fetches stored files and saves them to disk
```

