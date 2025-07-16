# RippleChat

**RippleChat** is a peer-to-peer, decentralized chat CLI application, built in Rust and powered by [iroh](https://github.com/n0-computer/iroh).

**RippleChat** offers secure and stable communication over QUIC. 

## Features

- Peer-to-Peer
**RippleChat** connects users directly without central servers. Node/User discovery and connections built upon iroh's DNS + Pkarr system.

- Signed Messages
Messages are signed with each user's private key.

- Relay Server Fallback
If direct communication fails, **RippleChat** uses iroh's relay infrastructure.

- Multi-User Chatrooms
Implemented through the gossip protocol.

- NAT Traversal via iroh
Allowing users behind routers, firewalls, or private networks to connect.