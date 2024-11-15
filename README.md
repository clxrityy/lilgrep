# lilgrep

A miniature version of the [grep](https://www.gnu.org/software/grep/manual/grep.html) command line tool built with Rust.

Search for a specific word within a file, and the entire line(s) containing that word will be returned to you.

- [Basic usage](#basic-usage)
- [Case sensitivity](#case-sensitivity)
- [Directing the output to a file](#directing-the-output-to-a-file)


##### Requirements:
- Must have Rust installed on your OS

```zsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Basic usage

- Search for the word *what* within the file `poem.txt`:

```zsh
cargo run what poem.txt
```

### Case sensitivity

By default, the search is case sensitive.
> If **"What"** exists but not the search query **"what"**, nothing will be returned.

##### This is dependant upon this environment variable:

```env
CASE_INSENSITIVE=
```

- To change this, all you need to do is run:

```zsh
export CASE_INSENSITIVE=true
```

- To remove:

```zsh
unset CASE_INSENSITIVE
```

---

## Directing the output to a file

- Search for every case of **"what"** within `poem.txt`, and direct the result to `output.txt`

```zsh
cargo run what poem.txt > output.txt
```