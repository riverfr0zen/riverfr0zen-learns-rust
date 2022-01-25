Fiddling around with Rust


# Rust installation

`rustup` is the primary management toolchain for Rust, and the [install page is here](https://www.rust-lang.org/tools/install).


# VSCode extensions

```
vscode --install-extension matklad.rust-analyzer
vscode --install-extension vadimcn.vscode-lldb
``` 

In addition the following was added to `.vscode/settings.json` to format code on save.

```
{
    "[rust]": {
        "editor.formatOnSave": true
    },
}
```


See also the [Bevy exploration README](explore-bevy-book/README.md) for other installation requirements.


# Further details around dev environment and tooling 

Below pulled from my project for learning Rust w/ Bevy.


## Rust core tooling
 
### rustup

`rustup` is the primary management toolchain for Rust, and the [install page is here](https://www.rust-lang.org/tools/install).
 
 `rustup self update` updates rustup itself
 
 `rustup update` will update the Rust installation

### cargo

[cargo](https://doc.rust-lang.org/cargo/index.html) is the Rust package manager

`cargo new hello_world` start a new package called hello_world

```Cargo defaults to `--bin` to make a binary program. To make a library, we would pass `--lib`, instead.```


`cargo build` build the project, which (if a binary crate) can then be run at `./target/debug/hello_world`

`cargo run` build and run the project in one go

`cargo update` update dependencies


## Rust language server choice & VSCode extension

[RLS](https://github.com/rust-lang/rls) VS [rust-analyzer](https://rust-analyzer.github.io/) 

Based on [this thread](https://www.reddit.com/r/rust/comments/lur37d/why_is_rls_still_so_bad/) and [this thread](https://www.reddit.com/r/rust/comments/hf07lk/rls_vs_rustanalyzer_in_2020/) I will go with `rust-analyzer`.

This decision also impacts choice of VSCode extension -- according to discussion in the threads above, if using `rust-analyzer`, one shouldn't have the default Rust VSCode extension running (conflicts and slowdown).

In addition, I also installed the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) debugger that I had seen recommended for Rust projects in VSCode. See [debug config in VSCode](https://bitbucket.org/riverfr0zen/rust-learning/src/master/.vscode/launch.json).


Some articles that contributed to my setup:

* https://dev.to/cthutu/rust-1-creating-your-development-environment-55bi
* https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
* https://devjunhong.github.io/rust/Rust-step-by-step-debugger/#seventh
* https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md#cargo-support    


## Code organization

### Workspace for rust-learning

https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

I already had a `/rust` folder containing a `hello-rust` package from a previous attempt. Upon reading about workspaces at the link above, it made sense then to make the `/rust` folder a "workspace" that can contain various learning projects (as packages). 

I renamed `/rust` to `/rust-learning` and created the necessary Cargo.toml file. Within this workspace, I can add upcoming projects, such as `explore-bevy-book` (learning Bevy) as `packages`.

I committed the entire `rust-learning` workspace [to Bitbucket](https://bitbucket.org/riverfr0zen/rust-learning)

One benefit of using this workspace structure is that it simplifies my [debug config in VSCode](https://bitbucket.org/riverfr0zen/rust-learning/src/master/.vscode/launch.json). 


## Enabling fast compiles on Bevy projects

Several methods to speed up compilation are listed at this [resource](https://bevyengine.org/learn/book/getting-started/setup/). So far, I have implemented the following:

* Enable Bevy's Dynamic Linking Feature (See [explorations Cargo.toml](explorations/Cargo.toml))

There are additional things that can be done, but so far this has been enough for me. Maybe as my projects become more complex I can revisit.

# Issues log
 
## Upgrading to Bevy 0.6

Was getting an error such as this:

```
   Compiling bevy_audio v0.5.0 (https://github.com/bevyengine/bevy.git?branch=main#e8412df0)
error: Metal API enabled on non-Apple OS. If your project is not using resolver="2" in Cargo.toml, it should.
  --> /home/keval/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-hal-0.11.4/src/lib.rs:49:1
   |
49 | compile_error!("Metal API enabled on non-Apple OS. If your project is not using resolver=\"2\" in Cargo.toml, it should.");
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: DX12 API enabled on non-Windows OS. If your project is not using resolver="2" in Cargo.toml, it should.
  --> /home/keval/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-hal-0.11.4/src/lib.rs:51:1
   |
51 | compile_error!("DX12 API enabled on non-Windows OS. If your project is not using resolver=\"2\" in Cargo.toml, it should.");
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

The issue is [described here](https://github.com/bevyengine/bevy/issues/3205). The fix mentioned there is to add a line to tell Rust which resolver to use.

However, because I have set up a workspace type of layout, in addition to adding the line `"resolver = 2"` (or, alternatively, `edition = "2021"` to each package in the workspace, the resolver also had to be specified in the workspace's Cargo.toml file. But in the workspace's Cargo.toml, it *has* to be " `"resolver = 2"`, NOT `edition = "2021"`. [This page](https://issueexplorer.com/issue/rust-lang/cargo/9956) goes into it.