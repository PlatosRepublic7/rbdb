# RBDB – A Simple Rust CLI Key-Value Store

**RBDB** (Rust-Based Database) is a toy CLI key-value store built in Rust. It demonstrates basic Rust concepts such as:
- Parsing user input with `read_line()`.
- Using enums to represent different query types (`INSERT`, `SELECT`, `UPDATE`, `DELETE`).
- Managing optional data via `Option<String>`.
- Storing data in a `HashMap<String, String>`.

It’s a beginner-friendly demonstration for learning Rust in a project-driven manner.

## Table of Contents
1. [Project Overview](#project-overview)
2. [Features](#features)
3. [Usage](#usage)
4. [Code Structure](#code-structure)
5. [Example Session](#example-session)
6. [Testing](#testing)
7. [Contributing](#contributing)
8. [License](#license)

---

## Project Overview

**Why this project?**

- It’s a small program that lets you explore how Rust handles data, ownership, error handling, and more.
- It’s an easy way to practice reading user input, building and matching enums, and manipulating data stored in a `HashMap`.

**What does it do?**

- Launches an interactive prompt (`RBDB -> `).
- Accepts query-like commands (`INSERT key value`, `SELECT key`, `UPDATE key value`, `DELETE key`).
- Performs basic operations on a simple in-memory store (`HashMap<String, String>`).
- Prints the results back to the user.

**Project Goals**

- File-based storage for key-value store (Start with String-based, followed later by binary-encoding).
- Multiple database functionality (Including full CLI support, prompt-reflected selection, etc).
- Multi-user integration (Password protection, user selection with their own databases, etc).
- Extend supported types (Lists, JSON, etc).
- Transition to network-based interaction (RBDB server/service). --> Long Term
- Custom Binary Protocol. --> Long Term

---

## Features

- **Enum-based queries:** Distinguish operations by creating a `QueryType` enum (`Insert`, `Select`, `Update`, `Delete`) to show idiomatic Rust patterns.
- **Graceful error handling:** Return errors with `Result` and bubble them up, or handle them inline with `match` or `if let`.
- **Optional values:** Demonstrate how to handle optional data with `Option<String>` for queries that sometimes have a value and sometimes don’t.
- **Interactive CLI:** The user can type commands repeatedly without restarting the program.
- **Extensible design:** Add more command types (e.g., `CREATE`, `DROP`, etc.) by simply adding new enum variants.

---

## Usage

1. **Clone** this repository:
   ```bash
   git clone https://github.com/your-username/rbdb.git
   ```
2. **Navigate** into the project directory:
   ```bash
   cd rbdb
   ```
3. **Build** the project:
   ```bash
   cargo build
   ```
4. **Run** the project:
   ```bash
   cargo run
   ```
5. **Enter** commands at the prompt (`RBDB -> `).

### Commands

- **INSERT key value**  
  Inserts a new entry into the store.  
  E.g., `INSERT username alice`
- **SELECT key**  
  Prints the value associated with `key`, if any.  
  E.g., `SELECT username`
- **UPDATE key value**  
  Updates the entry’s value if `key` exists; otherwise prints an error.  
  E.g., `UPDATE username bob`
- **DELETE key**  
  Removes the entry with the given `key`.  
  E.g., `DELETE username`
- **quit** or **exit**
  Exits the program.

---

## Code Structure

```
rbdb
├── src
│   ├── main.rs        (Entry point, driver functions)
│   └── lib.rs         (Main functionality)
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```

### `main.rs`

- Defines `main()` (entry point).
- Calls a `rbdb_run()` function that handles:
  - Prompting the user for input in a loop.
  - Parsing tokens into a `Query` via `Query::build_query(...)`.
  - Processing the query by matching on the `QueryType` enum.
  - Mutating or reading from a `HashMap` as needed.

### `Query` Type

```rust
enum QueryType {
    Insert,
    Select,
    Update,
    Delete,
}

struct Query {
    q_type: QueryType,
    key: String,
    value: Option<String>,
}

impl Query {
    pub fn build_query(tokens: Vec<&str>) -> Result<Self, Box<dyn Error>> {
        // ...
    }
}
```

### Processing Queries

```rust
fn process_query(query: &Query, store: &mut HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let mut query_result = String::new();
    match query.q_type {
        QueryType::Insert => { /* ... */ }
        QueryType::Select => { /* ... */ }
        QueryType::Update => { /* ... */ }
        QueryType::Delete => { /* ... */ }
    }

    OK(query_result)
}
```

---

## Example Session

An example run might look like this:

```
$ cargo run
   Compiling rbdb v0.1.0 (/path/to/rbdb)
    Finished dev [unoptimized + debuginfo] target(s) in 1.48s
     Running `target/debug/rbdb`

RBDB -> INSERT username alice
SUCCESS: Inserted username:alice into database

RBDB -> SELECT username
alice

RBDB -> UPDATE username bob
SUCCESS: Updated username with bob

RBDB -> SELECT username
bob

RBDB -> DELETE username
SUCCESS: Deleted username

RBDB -> SELECT username
No entry found for key = username

RBDB -> quit
```

---

## Testing

An example of tests check that `build_query` performs properly, and handles too few tokens:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_query() {
        let tokens = vec!["insert", "key", "value"];
        let query = Query::build_query(tokens.clone()).unwrap();
        assert_eq!(query.q_type, QueryType::Insert);
        assert_eq!(query.key, tokens[1]);
        let value = query.value.as_deref().unwrap();
        assert_eq![value, tokens[2]];
    }

    #[test]
    fn bad_query() {
        let tokens = vec!["delete"];
        let query = Query::build_query(tokens);
        assert!(query.is_err());
        let query_err = query.unwrap_err();
        assert_eq!(query_err.to_string(), "Not enough arguments");
    }
    // ...
}
```

Run tests with:
```bash
cargo test
```

---

## Contributing

1. Fork this repository.
2. Create a feature branch (`git checkout -b feature/new-thing`).
3. Commit your changes (`git commit -am 'Add new thing'`).
4. Push to the branch (`git push origin feature/new-thing`).
5. Create a new Pull Request in GitHub.

---

## License

[MIT License](LICENSE) © 2025 Ryan Kitson

This project is open-source, and you’re free to use or modify it as you like, subject to the MIT License.

---