* https://copy.sh/life/
* https://conwaylife.com/

```powershell
cargo build --workspace --verbose

cargo clean
cargo build --workspace --verbose


cargo build --workspace --release
cargo test --workspace
cargo build --workspace -j 8  # by default but explicit here 


```