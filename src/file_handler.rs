use crate::person::{EducationLevel, Gender, MaritalStatus, Person};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub fn save_to_file(filename: &str, persons: &[Person]) -> io::Result<()> {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(filename)?;

  for person in persons {
    writeln!(
      file,
      "{},{},{},{},{:.2},{},{},{},{};",
      person.id,
      person.name,
      person.lastname,
      person.age,
      person.salary,
      person.profession,
      person.gender.to_string(),
      person.education_level.to_string(),
      person.marital_status.to_string()
    )?;
  }

  Ok(())
}

pub fn load_from_file(filename: &str) -> io::Result<Vec<Person>> {
  let file = File::open(filename)?;
  let reader = BufReader::new(file);

  let mut persons = Vec::new();
  for line in reader.lines() {
    let line = line?;
    let fields: Vec<&str> = line.split('|').collect();

    if fields.len() == 9 {
      let person = Person {
        id: fields[0].parse().unwrap_or_default(),
        name: fields[1].to_string(),
        lastname: fields[2].to_string(),
        age: fields[3].parse().unwrap_or_default(),
        salary: fields[4].parse().unwrap_or_default(),
        profession: fields[5].to_string(),
        gender: Gender::from_string(fields[6]).unwrap_or(Gender::Other),
        education_level: EducationLevel::from_string(fields[7])
          .unwrap_or(EducationLevel::HighSchool),
        marital_status: MaritalStatus::from_str(fields[8]).unwrap_or(MaritalStatus::Single),
      };
      persons.push(person);
    }
  }
  Ok(persons)
}
