# personal_ai
Before using: ```npm install```

## Prerequisites
- Download `ollama` from https://ollama.ai/download
- Run `ollama pull neural-chat`

And you're all set!

## What you'll need
#### Dev
```
cargo run
```

#### Tests
you really don't want it to have more than two threads.
```
cargo test -- --test-threads=1
````

#### Build
```
cargo build
```
#
## Note:
The `node` version specified in `.nvmrc` is not supposed to change.

Because of a punycode warning thing in `v20.10.0+`, I just downgraded to the specified version.
