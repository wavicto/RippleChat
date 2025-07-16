# RippleChat

**RippleChat** is a peer-to-peer, decentralized chat CLI application, built in Rust and powered by [iroh](https://github.com/n0-computer/iroh). 

It offers secure and stable communication over QUIC. 

## Features

- **Peer-to-Peer Connections over QUIC**

**RippleChat** connects users directly without central servers. Node/User discovery and connections are built upon iroh's DNS + Pkarr system.

- **Signed Messages**

Messages are signed with each user's private key.

- **Relay Server Fallback**

If direct communication fails, **RippleChat** uses iroh's relay infrastructure.

- **Multi-User Chatrooms**

Implemented through the gossip protocol.

- **NAT Traversal via iroh**

Allowing users behind routers, firewalls, or private networks to connect.

## Build Instructions

```bash
git clone https://github.com/wavicto/ripplechat.git
cd ripplechat
cargo run --release
```
## Commands
Here are the commands used to navigate **RippleChat**

```bash
/help             Displays this list of commands.
/open             Opens a new chatroom. Shares a ticket that others can use to join.
/join [ticket]    Joins an existing chatroom using a ticket.
/leave            Leaves the currently joined chatroom.
/exit             Closes the application (you must /leave the chatroom first).
```
## Dependencies

- Rust (1.75+)
- Internet connection for DNS-based discovery
- `cargo` installed
