Learning about examples from http://xion.io/post/code/rust-examples.html

You can run the examples in this folder by calling `cargo run --example <example_name>` where <example_name> is the name of the `.rs` example file you want to run. For e.g.

```
cargo run --example hello
```

will run the main function in the `hello.rs` file found alongside this README.

Interestingly you can do the `cargo run` command above even from the top package level, so I'm not sure yet how it deals with conflicts in example names. In the Bevy project, examples are explicitly given a path in the top level Cargo.toml file.