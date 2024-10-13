## to install dioxus

```sh
cargo install dioxus-cli
```

```sh
dx new
```
## rules of hooks
1. hooks may only be used in components or other hooks
2. on every call to the component function the same hooks must be called and in the same order so hooks cannot be used in conditionals, closures or loops
3. hooks names shoul start with use_

## for format
#![allow(non_snake_case)]

on top of page

## Spawn
you should use spawn when performing things not tied to the component life cycle

## Corutine 
is for longrunning async actions like websockets