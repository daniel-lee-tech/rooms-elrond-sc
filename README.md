# Rooms

A way to match users on the Elrond Blockchain.

This is still a prototype.

## How it works

Users call the public `wait` method.

They will enter a list of waiting users. (which is a linked list, specifically a elrond-rust-wasm LinkedListMapper).

Once the length of the list reaches the `match_size` which is set on `init` it will match the first two users in the list and give them a match id. There match id is stored in `matches_map` which can be viewed with the method `getMatchesMap`. There match id will be tied to their elrond address so to call the `getMatchesMap` they must provide their address.
