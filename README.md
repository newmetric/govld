# newmetric/govld

'govld' is a code tool that adds/replaces code 'Go' according to a pre-written manifest files.
(Originally created only to modify files within the vendor folder, but now all files are targeted.)

## How to Install?

```bash
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install govld
CARGO_NET_GIT_FETCH_WITH_CLI=true cargo install govld --git https://github.com/newmetric/govld
```

## TODO: haha

```sh
patch:jest-message-util@28.1.3#.yarn/patches/add-path.patch
patch:jest-message-util@npm:28.1.3#.yarn/patches/add-path.patch
patch:jest-message-util@npm:^28.1.3#.yarn/patches/add-path.patch
patch:grunt@github:gruntjs/grunt#.yarn/patches/fix-exec.patch
```
