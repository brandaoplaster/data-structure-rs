pub struct Person {
  pub id: u32,
  pub name: String,
  pub lastname: String,
  pub age: u32,
  pub salary: f64,
  pub profession: String,
  pub gender: Gender,
  pub education_level: EducationLevel,
  pub marital_status: MaritalStatus,
}

pub enum Gender {
  Male,
  Female,
  Other,
}

pub enum MaritalStatus {
  Single,
  Married,
  Divorced,
}

pub enum EducationLevel {
  HighSchool,
  Bachelor,
  Master,
  PhD,
}
