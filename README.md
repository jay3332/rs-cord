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
    </sup>
</p>

----

## Navigation
- [Motive](#motive)
- [Installation](#installation)
- [Getting Started](#getting-started)

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

## Getting Started
Example Bot:
```rs
use rs_cord::{Client, ClientState, CacheConfiguration, EventListener};
use rs_cord::events::{ReadyEvent, MessageCreateEvent}

#[macro_use]
use rs_cord::macros::intents;

struct Listener;

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
    Client::new_with_token(TOKEN)
        .with_intents(intents!(messages, reactions))
        .with_event_listener(Listener)
        .run()
        .await
        .expect("failed to start client");
}
```
