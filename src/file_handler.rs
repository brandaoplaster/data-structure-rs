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
      "{}|{}|{}|{}|{:.2}|{}|{}|{}|{}",
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
