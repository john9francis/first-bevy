# My First Bevy Game

# How to start developing a bevy game

Create a new rust project
```
cargo new my_bevy_game
```

Add bevy as a dependency
```
cargo add bevy
```

How to run:
```
cargo run
```

How to compile for release:
```
cargo build --release
```

# Dynamic linking

Enabling dynamic linking can make compilation a lot faster. 
I just need to remember to disable it before releasing,
otherwise the game can't be standalone. So to remove
dynamic linking, I need to change this line:
```
bevy = { bevy = "0.13.0", features = ["dynamic_linking"] }
```
to:
```
bevy = "0.13.0"
```

# Progress:
- 2/18/24 got as far as [create your own plugin](https://bevyengine.org/learn/quick-start/getting-started/plugins/)
- 3/7/24 Finished the tutorial, ended [here](https://bevyengine.org/learn/quick-start/next-steps/)

# Useful Websites:
- [Getting started with bevy](https://bevyengine.org/learn/quick-start/getting-started/setup/)
- [More bevy resources](https://bevyengine.org/learn/quick-start/next-steps/)