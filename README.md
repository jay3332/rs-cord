![rs-cord](https://i.ibb.co/jkBvVBR/rs-cord-png.png)
<h1 align="center">
    <b>rs-cord</b>
</h1>
<p align="center">
    <sup>
        A high-level Rust crate around the Discord API, aimed to be easy and straight-forward to use.
        <br>
        <a href="https://docs.rs/rs-cord"><b>Documentation</b></a> 
        • 
        <a href="https://crates.io/crates/rs-cord"><b>Crates.io</b></a>
        • 
        <a href="https://discord.gg/uch68Ujf38"><b>Discord</b></a>
    </sup>
</p>

----

## Navigation
- [Motive](#motive)
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Need help?](#need-help)
- [Contributing](#contributing)
- [Links](#links)

## Motive
Currenty, the leading crates that wrap around Discord's API (e.g. [serenity](https://github.com/serenity-rs/serenity)),
although some relatively high level, do not provide as much abstraction as I would like and requires lots of concepts -
ones that are common in other frameworks - to be done manually.

Take asset/avatar handling. [discord.py](https://github.com/Rapptz/discord.py) does this pretty well:

The equivalent of ``member.avatar.replace(format="png", size=2048)`` in an existing Rust crate, such as [serenity](https://github.com/serenity-rs/serenity) would be:
```rs
format!("https://cdn.discordapp.com/avatars/{}/{}.png?size={}", member.user.id, member.user.avatar, 2048)  // Verbose for what?
// or
member.face().replace(".webp", ".png") + "?size=2048"  // Not elegant
```

<sub>(For clarification, I have nothing against serenity; it's a pretty good crate.)</sub>

rs-cord has been designed with two things in mind: elegance and performance.

### Elegance

#### Assets
rs-cord takes a different approach to assets and avatars than serenity. The `Asset` struct will fulfill all your needs:
```rs
user.avatar().with_format("png").with_size(2048).url()
```

#### No more `member.user.x`
Discord's API has a nested `user` in `member` objects. 
Many Rust crates do not merge these two, which can cause confusion to the user.

rs-cord merges all fields from `User` into `Member`:
```rs
println!("Hello {}", member.name);
```

## Installation
Just like with every other Rust crate, insert it into your `Cargo.toml` file:
```toml
[dependencies]
rs-cord = "0"
```

### Installing from GitHub
If you would like to use the most recent update of rs-cord, make sure you have `git` installed and 
insert the following into your `Cargo.toml`:
```toml
[dependencies]
rs-cord = { git = "https://github.com/jay3332/rs-cord" }
```

Note that for production usage, it is recommended to stay with a stable version upload on [crates.io](https://crates.io/crates/rs-cord).

Documentation for the GitHub version is available [here](https://jay3332.github.io/rs-cord/).

## Getting Started
You may see the [examples folder](https://github.com/jay3332/rs-cord/tree/main/examples) for code examples.

Example Ping-pong Bot:
```rs
use rs_cord::{Client, ClientState, EventListener};
use rs_cord::events::{ReadyEvent, MessageCreateEvent};
use rs_cord::intents;

struct Listener;

#[rs_cord::async_trait]
impl EventListener for Listener {
    async fn ready(state: &ClientState, _event: &ReadyEvent) {
        println!("Logged in as {} (ID: {})", state.user.tag(), state.user.id);
    }

    async fn message_create(state: &ClientState, event: &MessageCreateEvent) {
        if event.message.content == "ping" {
            event.message.reply("Pong!");
        }
    }
}

#[tokio::main]
async fn main() {
    Client::new_with_token("my secret token")
        .with_intents(intents!(GUILDS, MESSAGES))
        .with_event_listener(Listener)
        .start()
        .await
        .expect("failed to start client");
}
```

## Need help?
You can [join our Discord server](https://discord.gg/uch68Ujf38) in order to get help on all things rs-cord.

## Contributing
Make sure to format your code using `cargo fmt`. Additionally, make sure it passes `cargo check` with minimal to zero warnings.

## Links
- [Documentation (Stable)](https://docs.rs/rs-cord)
- [Documentation (GitHub)](https://jay3332.github.io/rs-cord)
- [Crates.io](https://crates.io/crates/rs-cord)
- [Discord Server](https://discord.gg/uch68Ujf38)
- [Examples](https://github.com/jay3332/rs-cord/tree/main/examples)
