# Uno Game

## Prerequisites
```bash
sudo apt install libasound2-dev
sudo apt-get install -y  libudev-dev
```

## run
### client
```bash
cargo run -p game-client
```

### server
```bash
cargo run -p game-server
```

```
.
├── game-client               | game client implementation
│   ├── src
│   │   ├── game              |
│   │   │   ├── board.rs      | board state and rendering
│   │   │   ├── hand.rs       | player hand state and rendering
│   │   │   ├── hud.rs        | heads-up display state and rendering
│   │   │   ├── mod.rs        | game state management
│   │   │   └── pause.rs      | pause state and rendering
│   │   ├── i18n              | internationalization
│   │   │   └── mod.rs        | internationalization state and rendering
│   │   ├── menu              | menu state and rendering
│   │   │   ├── lobby.rs      | lobby state and rendering
│   │   │   ├── main_menu.rs  | main menu state and rendering
│   │   │   ├── mod.rs        | menu state management
│   │   │   └── settings.rs   | settings state and rendering
│   │   ├── network           | network state and rendering
│   │   │   ├── mod.rs        | network state management
│   │   │   └── socket.rs     | network socket state and rendering
│   │   ├── main.rs           | main state and rendering
│   │   ├── settings.rs       | settings state and rendering
│   │   ├── states.rs         | state management
│   │   └── ui.rs             | UI state and rendering
│   └── Cargo.toml            |
├── game-core                 | game logic
│   ├── src                   |
│   │   ├── action.rs         |
│   │   ├── card.rs           |
│   │   ├── error.rs          |
│   │   ├── event.rs          |
│   │   ├── lib.rs            |
│   │   ├── player.rs         |
│   │   └── state.rs          |
│   └── Cargo.toml            |
├── game-protocol             | shared network types
│   ├── src                   |
│   │   ├── client.rs         |
│   │   ├── lib.rs            |
│   │   ├── lobby.rs          |
│   │   ├── server.rs         |
│   │   └── view.rs           |
│   └── Cargo.toml            |
└── game-server               | game server implementation
    ├── src                   |
    │   ├── lobby.rs          |
    │   ├── main.rs           |
    │   ├── network.rs        |
    │   ├── server_state.rs   |
    │   └── systems.rs        |
    └── Cargo.toml            |
```
