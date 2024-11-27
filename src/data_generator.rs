use crate::person::{EducationLevel, Gender, MaritalStatus, Person};
use fake::{
  faker::{
    boolean::en::Boolean,
    name::en::{FirstName, LastName},
  },
  Fake,
};
use rand::{seq::SliceRandom, thread_rng};

const PROFESSIONS: &[&str] = &[
  "Software Engineer",
  "Product Manager",
  "Data Scientist",
  "Designer",
  "Marketing Specialist",
  "Teacher",
  "Doctor",
  "Nurse",
  "Mechanic",
  "Electrician",
];

pub fn generate_fake_person(id: u32) -> Person {
  let gender = if Boolean(50).fake() {
    Gender::Male
  } else {
    Gender::Female
  };

  Person {
    id,
    name: FirstName().fake(),
    lastname: LastName().fake(),
    age: (18..65).fake(),
    salary: (30_000.0..120_000.0).fake(),
    profession: random_element(PROFESSIONS).unwrap().to_string(),
    gender,
    education_level: generate_random_education_level(),
    marital_status: generate_random_marital_status(),
  }
}

pub fn generate_fake_people(count: u32) -> Vec<Person> {
  (1..=count).map(generate_fake_person).collect()
}

fn generate_random_marital_status() -> MaritalStatus {
  match (0..3).fake() {
    0 => MaritalStatus::Single,
    1 => MaritalStatus::Married,
    _ => MaritalStatus::Divorced,
  }
}

fn generate_random_education_level() -> EducationLevel {
  match (0..4).fake() {
    0 => EducationLevel::HighSchool,
    1 => EducationLevel::Bachelor,
    2 => EducationLevel::Master,
    _ => EducationLevel::PhD,
  }
}

pub fn random_element<T>(list: &[T]) -> Option<&T> {
  let mut rng = thread_rng();
  list.choose(&mut rng)
}
