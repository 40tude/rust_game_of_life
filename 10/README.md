## Can pass .rle as argument

* `cargo add clap  -p step_10`

* `cargo run -p step_10 -- -help`
* `cargo run -p step_10 -- --version`

* `cargo run -p step_10 -- --pattern do_not_exist`
* `cargo run -p step_10 -- -p do_not_exist`

* `cargo run -p step_10 -- --pattern rle/spaceships`
* `cargo run -p step_10 -- -p rle/spaceships`

## Notes
*  PathBuf is owned and mutable where Path is an immutable view
* Let's fix the vocabulary:

| Concept       | Type Rust           | Exemple               | Content                |
| ------------- | ------------------- | --------------------- | ----------------------- |
| **Path**      | `&Path` / `PathBuf` | `/home/user/file.txt` | The whole path to file     |
| **Parent**    | `Option<&Path>`     | `/home/user`          | Part before the file name |
| **File name** | `Option<&OsStr>`    | `file.txt`            | Name + extension         |
| **File stem** | `Option<&OsStr>`    | `file`                | Name without extension      |
| **Extension** | `Option<&OsStr>`    | `txt`                 | Extension alone         |



## TODO:
* Use GPU ?
* Rendering in a thread such that it continue while we move the window on screen
* Add more comments/documentation
* Add more tests

## DONE
* Load pattern from the command line
* Load pattern pressing 'O'
* utils::read_rle() should support patterns file without x and y
* Handle errors in place_pattern_centered() and when returning from place_pattern_centered()
* Remove cells_in_corners()
* Translate initial comments FR->US
* Split in modules and lib

## ??
* https://copy.sh/life/examples/