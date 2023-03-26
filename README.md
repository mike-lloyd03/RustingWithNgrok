# Rusting with Ngrok

## Summary

This is a Rust rewrite of the websocket chat app created by Shub Argha demonstrated [here](https://shub.codes/ngrokchat2/).

## Usage

This chat app is served by [`ngrok`](https://ngrok.com/). Running the application will publish it on the internet. Authentication is done via OAuth with Google as the identity provider.

The app can be run with `cargo run`. Upon initialization, it will return the `ngrok` URL that the app is being served at. Navigating to it will start the OAuth flow and once done, you'll be granted access to the app.

## Why

Seemed like a fun way to learn about websockets and everything should be written in Rust anyway so I'm just doing my part.
