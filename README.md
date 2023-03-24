# Rusting with Ngrok

## Summary

This is a Rust rewrite of the websocket chat app created by Shub Argha demonstrated [here](https://shub.codes/ngrokchat2/).

## Usage

This chat app is meant to be served by [`ngrok`](https://ngrok.com/) and can easily be served over the internet with a one-line command.

First build and run the app:

```bash
cargo run
```

Once it's up, run:

```bash
ngrok http 8000 --oauth=google
```

`ngrok` will return the URL it is serving the app at. Navigating to it will start the Oauth flow and once done, you'll be granted access to the app.

## Why

Seemed like a fun way to learn about websockets and everything should be written in Rust anyway so I'm just doing my part.
