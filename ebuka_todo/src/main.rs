use serde:: {Deserialize, Serialize};
use std::fs;
use std::path::Path;
use dialoguer:: { theme::ColorfulTheme, Input, Select};
use colored::*;
use uuid::Uuid;
use std::error::Error;
use std::fs::File;
use csv::Writer;
use std::io::Write;



const TODO_FILE: &str = "todos.json";

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    id:Uuid,
    description:String,
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

        TodoList { items }
    }

    fn save (&self) {
        let data = serde_json::to_string(&self.items).expect("Failed to serialize");
        fs::write(TODO_FILE, data).expect("Failed to write data");
    }


    fn add_item(&mut self, description: String) {
        let item = TodoItem {
            id : Uuid::new_v4(),
            description,
        };

        self.items.push(item);
        self.save();
        println!("{}","An Item was added".green());

    }

    fn list_items(&self) {
        if self.items.is_empty() {
            println!("{}","No todo items found".yellow());
        } else {
            println!("Todo list:");

            for item in &self.items {
                println!("{}: {}", item.id, item.description);
            }


        }
    }

    fn remove_item(&mut self, id:Uuid) {
        if let Some(pos) = self.items.iter().position(|x| x.id == id) {
            self.items.remove(pos);
            self.save();
            println!("{}",format!("Todo item {} removed", id).green());
        } else {
            println!("{}",format!("There's no item found with id {}", id).red());
        }
    }
    
    

    fn export_to_csv(&self, mut filename: String) -> Result<(), Box<dyn Error>> {
        if Path::new(&filename).extension().is_none() || Path::new(&filename).extension().unwrap() != "csv" {
            filename.push_str(".csv");
        }

        let file = File::create(&filename)?;
        let mut wtr = Writer::from_writer(file);
        

        for item in &self.items {
            wtr.serialize(item)?;
        }
        wtr.flush()?;
        println!("{}",format!("Exported the todos to {}", filename).green());
        Ok(())

    }




    fn import_csv(&mut self, filename: &str) -> Result<(), Box<dyn Error >> {
        let file = File::open(filename)?;
        let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(file);

        for result in rdr.deserialize() {
            let item: TodoItem = result?;

            self.items.push(item);
        }

        self.save();
        println!("{}", format!("The Imported todos from {}", filename).green());
        Ok(())
    }

    fn export_markdown(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        writeln!(file, "# ToDo List\n")?;
        for item in &self.items {
            writeln!(file, "- [] {}: {}", item.id, item.description)?;
        }

        println!("{}",format!("Exported the todos to {}", filename).green());
        Ok(())
    }

    fn import_markdown(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        for line in content.lines() {
            if let Some(desc_start) = line.find("]") {
                let description = line[desc_start + 2..].to_string();

                self.add_item(description);
            }
        }

        println!("{}", format!("Imported todos from {}", filename).green());
        Ok(())
    }
}


fn main() {
    let mut todo_list = TodoList::new();

    let theme = ColorfulTheme::default();

    loop {
        println!("\n--- Ebuka ToDo CLI ---");

        let options = &[
            "Add ToDo",
            "List ToDo",
            "Remove ToDo",
            "Export ToDo",
            "Import ToDo",
            "Quit",
        ];

        let selection = Select::with_theme(&theme).with_prompt("hey select an option").items(&options[..]).default(0).interact().unwrap();

        match selection {
            0 => {
                let description: String = Input::with_theme(&theme).with_prompt("What's your todo for today?").interact_text().unwrap();

                if description.trim().is_empty() {
                    println!("{}","The Description cannot be empty".red());
                } else {
                    todo_list.add_item(description);
                }
            },

            1 => {
                
                todo_list.list_items();
            },

            2 => {
                
                if todo_list.items.is_empty() {

                    println!("{}","There's nothing to remove".yellow());

                    continue;

                }

                let choices: Vec<String> = todo_list.items.iter().map(|item| format!("{}: {}", item.id, item.description)).collect();

                let selection = Select::with_theme(&theme).with_prompt("Select the TODO you want to improve").items(&choices).default(0).interact();

                match selection {
                    Ok(index) => {
                        let id = todo_list.items[index].id;
                        todo_list.remove_item(id);
                    },

                    Err(_) => println!("{}","You didn't make any selection".yellow()),
                }
            },

            3 => {
                let export_options = &["Export to CSV", "Export to Markdown"];

                let export_selection = Select::with_theme(&theme).with_prompt("Select the export option").items(&export_options[..]).default(0).interact().unwrap();

                let filename: String = Input::with_theme(&theme).with_prompt("Enter filename to export to").interact_text().unwrap();

                match export_selection {
                    0 => {
                        

                        if ! filename.ends_with(".csv") {

                            println!("{}","Filename extension '.csv' was not provided. It has been appended automatically".yellow());
                        }

                        if let Err(e) = todo_list.export_to_csv(filename) {
                            println!("{}", format!("Failed to export to CSV: {}", e).red());
                        }
                    }, 

                    1 => {

                        if let Err(e) = todo_list.export_markdown(&filename) {
                            println!("{}", format!("Failed to export to Markdown: {}", e).red());
                        }
                    },

                    _ => unreachable!(),
                }
            },

            4 => {
                let import_options = &["Import from CSV", "Import from Markdown"];

                let import_selection = Select::with_theme(&theme).with_prompt("Select the import option").items(&import_options[..]).default(0).interact().unwrap();

                let filename: String = Input::with_theme(&theme).with_prompt("Enter filename to import from").interact_text().unwrap();

                match import_selection {
                    0 => {
                        if let Err(e) = todo_list.import_csv(&filename) {
                            println!("Failed to import from CSV: {}", e.to_string().red());
                        }
                    }, 

                    1 => {
                        if let Err(e) = todo_list.import_markdown(&filename) {
                            println!("Failed to import from Markdown: {}", e.to_string().red());
                        }
                    }, 

                    _ => unreachable!(),
                }
            },

            5 => {
                println!("{}","Goodbye".green());
                break;

            },

            _ => unreachable!(),
        }
    }

}
