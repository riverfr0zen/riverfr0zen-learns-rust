Fiddling around with Rust


# Rust installation

`rustup` is the primary management toolchain for Rust, and the [install page is here](https://www.rust-lang.org/tools/install).


# VSCode extensions

```
vscode --install-extension matklad.rust-analyzer
vscode --install-extension vadimcn.vscode-lldb
``` 

See also the [Bevy exploration README](explore-bevy-book/README.md) for other installation requirements.

 
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