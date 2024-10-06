use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use dialoguer:: {theme::ColorfulTheme, Input, Select};

const TODO_FILE: &str = "todos.json";

#[derive(Serialize, Deserialize, Debug)]

struct TodoItem {
    id: usize,
    description: String,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        let items = if Path::new(TODO_FILE).exists() {
            let data = fs::read_to_string(TODO_FILE).expect("Unable to read file");

            serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())

        } else {
            Vec::new()
        };

        TodoList {items}
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self.items).expect("Failed to serialize todos.");
        fs::write(TODO_FILE, data).expect("Unable to write to todo file.");
    }

    // add a new item 
    fn add_item(&mut self, description: String) {
        let id = self.items.len() + 1;
        let item = TodoItem {id, description};
        self.items.push(item);
        self.save();
        println!("A Todo Item was added ");
    }

    fn list_items(&self) {
        if self.items.is_empty() {
            println!("No todo items found");
        } else {
            println!("Your todo list");

            for item in &self.items {
                println!("{}: {}", item.id, item.description);
            }
        }
    }

    // remove todo item via its ID 
    fn remove_item(&mut self, id: usize) {
        if let Some(pos) = self.items.iter().position(|x| x.id == id) {
            self.items.remove(pos);

            for (index, item) in self.items.iter_mut().enumerate() {
                item.id = index + 1;
            }

            self.save();
            println!("Todo item {} removed", id);
        } else {
            println!("No item found with id {}", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let theme = ColorfulTheme::default();

    loop {
        println!("\n--- ToDO CLI ---");
        let options = &[
            "Add Todo",
            "List Todos",
            "Remove Todo",
            "Quit",
        ];

        let selection = Select::with_theme(&theme)
            .with_prompt("Select an option")
            .items(&options[..])
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let description: String = Input::with_theme(&theme).with_prompt("What is your todo?").interact_text().unwrap();

                if description.trim().is_empty() {
                    println!("Description cannot be empty");
                } else {
                    todo_list.add_item(description);
                }
            }, 
            1 => {
                todo_list.list_items();
            },

            2 => {
                if todo_list.items.is_empty() {
                    println!("No todo items to remove");
                    continue;
                }

                let choices: Vec<String> = todo_list.items.iter().map(|item| format!("{}: {}", item.id, item.description)).collect();

                let selection = Select::with_theme(&theme).with_prompt(
                    "Select a TODO to rmove"
                ).items(&choices).default(0).interact();

                match selection {
                    Ok(index) => {
                        let id = todo_list.items[index].id;

                        todo_list.remove_item(id);
                    }, 
                    Err(_) => println!("No selection made"),
                }
            },

            3 => {
                println!("Goodbye!");
                break;
            },
            _ => unreachable!(),
            
        }
    }
}
