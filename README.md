# 🏆 Sports Management System 🏅

## Overview

This is a decentralized Sports Management System built on the Internet Computer (IC) blockchain platform using Rust. The system provides comprehensive functionality for managing sports teams, matches, users, and more.

## 🌟 Features

### User Management

- 👤 User registration and profile management
- 🔐 Role-based access (Player, Coach, Admin)
- 📧 Email validation and uniqueness checks

### Team Management

- � チ Create and manage sports teams
- 👥 Add players to teams
- 🧑‍🏫 Assign coaches to teams

### Match Management

- 📅 Schedule matches between teams
- 🏟️ Support for multiple sport types
- 📊 Submit and track match results

## 🛠️ Technology Stack

- **Language**: Rust
- **Platform**: Internet Computer (IC)
- **Key Libraries**:
  - `candid`: For type definition and serialization
  - `ic-stable-structures`: For persistent storage
  - `regex`: For email validation

## 🚀 Key Components

- `User`: Represents system users with roles
- `Team`: Represents sports teams with members and coaches
- `Match`: Represents scheduled and completed matches
- `Referee`: (Placeholder for future implementation)
- `Tournament`: (Placeholder for future implementation)
- `League`: (Placeholder for future implementation)

## 📦 Data Storage

Uses `StableBTreeMap` for persistent, stable storage of:

- Users
- Teams
- Matches
- Referees
- Tournaments
- Leagues

## 🔍 Main Functions

- `register_user`: Create new user profiles
- `create_team`: Form new sports teams
- `schedule_match`: Arrange matches between teams
- `add_member`: Add players to teams
- `assign_coach`: Assign coaches to teams
- `submit_match_result`: Record match outcomes

## 🌐 Deployment

Designed to be deployed on the Internet Computer blockchain, ensuring:

- Decentralization
- Data integrity
- Transparent record-keeping

## 🔒 Security Features

- Caller authentication
- Role-based access control
- Email format and uniqueness validation
- Unique ID generation

## Requirements

- rustc 1.64 or higher

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```

- rust wasm32-unknown-unknown targetz

```bash
$ rustup target add wasm32-unknown-unknown
```

- candid-extractor

```bash
$ cargo install candid-extractor
```

- install `dfx`

```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ git clone https://github.com/aarontsimamgo/inter-uni-sports-league.git
$ cd inter-uni-sports-league/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:

```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:

```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background --clean

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
