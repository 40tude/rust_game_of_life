The associated [post](https://www.40tude.fr/docs/06_programmation/rust/017_game_of_life/game_of_life_00.html) in plain franglish.


Note : 

```powershell
cargo clean
cargo build --workspace --verbose

cargo build --workspace --release
cargo test --workspace
cargo build --workspace -j 8  # by default but explicit here 
```