use data_generator::generate_fake_people;
use file_handler::{load_from_file, save_to_file};
use person::Person;

mod data_generator;
mod file_handler;
mod person;

fn main() {
  let fake_persons: Vec<Person> = generate_fake_people(10);

  let filename = "fake_persons.txt";

  match save_to_file(filename, &fake_persons) {
    Ok(_) => println!("File saved successfully!"),
    Err(e) => println!("Error saving file: {}", e),
  }

  let loaded_persons = load_from_file(filename);
  println!("Dados carregados: {:?}", loaded_persons);
}
