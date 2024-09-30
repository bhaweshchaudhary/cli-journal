## CLI Journal

A command-line journal written in Rust for managing your daily entries. Easily add, view, and search for entries by date.

## Features

- **Add Entries**: Write and save your daily journal entries.
- **View Entries**: Browse through your past entries in a clear format.
- **Search Entries**: Find specific entries by date (YYYY-DD-MM format).

## Installation

**Prerequisites:**

- Rust compiler: Install Rust by running the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

## Steps:

1. **Clone the repository:**

```bash
git clone [https://github.com/bhaweshchaudhary/cli-journal.git](https://github.com/bhaweshchaudhary/cli-journal.git)
cd cli-journal
```

2. **Build the project:**

```bash
cargo build
```

3. **Run the project:**

```bash
cargo run
```

**Usage**
The program offers three options:

- Add Entry: Create a new journal entry for the day.
- View Entries: Display all your past entries.
- Search Entries: Find entries for a specific date (YYYY-DD-MM format).

**Dependencies**
This project uses the following Rust dependencies:

- chrono: For handling dates and times.
- std::io: For reading and writing data.
- std::fs: For managing files (reading from and writing to _journal.txt_).

**Contributing**
Feel free to contribute to this project! Fork the repository, submit issues, or create pull requests with improvements.

**License**
This project is licensed under the MIT License.

![](https://ibb.co/pRJdVZR)
