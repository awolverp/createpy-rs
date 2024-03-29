# CreatePy
A simple command-line tool to create Python projects, written in Rust.

Create Python projects, create a virtual environment, and create an empty git repository for it,
with only one command.

> [!NOTE]\
> This isn't a special repository, I wrote this tool to speedup my works, and practice **Rust** language.

**Features**:
- `Virtualenv` and `Venv` scripts are supported.
- `git` is supported:
    - Create an empty git repository, you can specify branch name,
    - Set user name, and email address for it,
    - Add new remote to it.

### Example
<p align=center>
    <img src="https://github.com/awolverp/createpy-rs/assets/118073811/69d2f0dd-c36c-4eb9-bd4b-53436363126e" width="90%"/>
</p>


## Installation
You can build the project from source, with [*rust compiler*](https://www.rust-lang.org/tools/install).

-----

**First way:**

1. Use the `cargo install` command:
```bash
cargo install --git 'https://github.com/awolverp/createpy-rs'
```
2. Now the tool is installed and you can use it:
```bash
createpy -h
```

> [!TIP]\
> You can uninstall it by using `cargo uninstall createpy` command

-----

**Second way:**

1. First, download source from here by using the `git clone` (or any tool you can use)
```bash
git clone 'https://github.com/awolverp/createpy-rs'
```
2. Go to the source directory.
3. Run this command:
```bash
cargo build --release
```
4. Now you can use this tool, the binary file is stored here: `./target/release/createpy`:
```bash
./target/release/createpy -h
```

## License
[**MIT License**](https://opensource.org/licenses/MIT)
