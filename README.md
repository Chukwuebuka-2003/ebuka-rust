# 🗒️ Ebuka Todo CLI App

A simple and interactive command-line interface (CLI) todo application written in Rust. 
Manage your daily tasks effortlessly using this todo list app.

![Screenshot from 2024-10-06 21-32-36](https://github.com/user-attachments/assets/5549ae72-8481-436e-8281-5acb0d921548)


## Features

- Add new todo items.
- List all your todos.
- Remove a todo item by selecting from the list.
- Persistent storage of todos in `todos.json`.
- Interactive user interface using the `dialoguer` crate.

## Installation

### Prerequisites

To build and run this application, you'll need:

- [Rust](https://www.rust-lang.org/tools/install) (make sure you have `cargo` installed)

### Steps

1. Clone the repository:

   ```bash
   git clone git@github.com:Chukwuebuka-2003/ebuka-rust.git
   cd ebuka_todo
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the application:

   ```bash
   cargo run
   ```

### Usage

Once the application is running, it will present you with a menu of options to manage your todos:

```bash
--- 🗒️  Todo CLI ---
> Add Todo
  List Todos
  Remove Todo
  Exit
```

You can interact with the app using the arrow keys to navigate and the Enter key to select an option.

## Commands

### 1. Add Todo

This option lets you add a new todo item. After selecting, you'll be prompted to enter a description for the task.

```bash
Enter todo description: Learn Rust basics
```

### 2. List Todos

Lists all the todo items currently in your list. Each todo item is displayed with an ID and its description.

Example output:

```bash
📝 Your Todo List:
1: Learn Rust basics
2: Work on a ML project
```

### 3. Remove Todo

Allows you to remove a todo item by selecting it from the list. You'll be presented with a list of tasks, and you can choose one to remove:

```bash
Select a todo to remove:
> 1: Learn Rust basics
  2: Build a CLI tool
```

After selecting an item, it will be removed, and the remaining todo items will be renumbered.

### 4. Exit

This option exits the application. All changes are automatically saved to `todos.json`, ensuring your todos persist across sessions.

## Persistent Storage

The todo items are stored in a `todos.json` file in the root directory of the project. This allows for persistence, so even if you close the app, your todos will remain intact when you run the app again.

## Dependencies

This project uses the following Rust crates:

- [`serde`](https://crates.io/crates/serde) for serializing and deserializing the todo items.
- [`serde_json`](https://crates.io/crates/serde_json) for reading and writing JSON files.
- [`dialoguer`](https://crates.io/crates/dialoguer) for creating an interactive CLI.

You can find the full list of dependencies in the [`Cargo.toml`](Cargo.toml) file.

## Future Enhancements

Planned features include:

- Marking tasks as completed.
- Editing existing todo descriptions.
- Prioritization of tasks (low, medium, high).
- Adding due dates for todos.

## Contributing

Feel free to fork this project and submit a pull request if you'd like to contribute or suggest improvements.

1. Fork the project.
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request


## Acknowledgements

- Thanks to the creators of the `serde` and `dialoguer` crates for making serialization and interactive CLIs easier to implement in Rust.
