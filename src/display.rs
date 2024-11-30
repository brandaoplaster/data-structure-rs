use std::io;

pub fn show_menu() {
  loop {
    print!("\n Main Menu ");
    print!("1.  \n");
    print!("2. \n");
    print!("3. \n");
    print!("4. \n");

    let choice = get_choice();
    match choice.as_str() {
      "1" => println!("Linked list"),
      "3" => {
        close_program();
        break;
      }
      _ => println!("Invalid option"),
    }
  }
}

fn get_choice() -> String {
  let mut choice = String::new();
  io::stdin()
    .read_line(&mut choice)
    .expect("Errror invalid option");
  choice.trim().to_string()
}

fn close_program() {
  println!("Invalid option");
}
