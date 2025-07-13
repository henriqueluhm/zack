# zack

**zack** is a terminal-based text editor written in Rust, built with [Ropey](https://github.com/cessen/ropey), [Crossterm](https://github.com/crossterm-rs/crossterm), and [Ratatui](https://github.com/tui-rs/ratatui).  
It is an early-stage experimental project focused on core editing functionality such as buffer manipulation and vim motions.

## MVP Status

zack currently offers a minimal, working editor experience:

- **Open a file** on startup using a CLI argument (`cargo run path/to/file.txt`)
- **Basic UI** indicating current mode (Insert or Normal) and file name prompt if not present on startup
- **Edit and save** changes to the file (or save as new if none was provided)
- **Normal mode** with Vim-style motions:
  - `h`, `j`, `k`, `l` to move
  - `i`, `a` to enter insert mode
  - `Ctrl + S` to save
- **Single buffer**

---

## Build & Run

```sh
cargo run
```

To open a file on launch:

```sh
cargo run path/to/file.txt
```

1. **Strong Unit Test Coverage**

2. **Documentation with `cargo doc`**

- Add comprehensive Rust documentation comments for all public modules, structs, and functions.
- Enable easier understanding for future maintenance.

3. **Robust Error Handling and Application State**

- Implement meaningful error propagation and handling throughout the codebase.
- Add UI elements such as status bars or notifications to display warnings, errors, and application states.

4. **Basic Continuous Integration (CI) Pipeline**

- Set up automated CI workflows (e.g., GitHub Actions) to run linting, formatting checks, and tests on every pull request.
- Increase development velocity and code quality with automated feedback.

5. **Download and Install Guide**

- Document how to build and install **zack** via `cargo install` or other methods.
- Prepare for wider distribution and easier adoption.
