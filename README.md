# Monarchess Discord bot
Project repository: https://github.com/ChromaCat248/monarchess

## Run instructions

Compile.  
```
cargo build
```

Copy config.yaml.sample into the build directory and rename it to "config.yaml".  
```
cp ./config.yaml.sample ./target/debug/config.yaml
```

Edit the new file to add a bot token.  
```
token: "[insert bot token here]"  
prefix: "m!"
```
