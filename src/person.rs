#[derive(Debug)]

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

#[derive(Debug)]

pub enum Gender {
  Male,
  Female,
  Other,
}

#[derive(Debug)]

pub enum MaritalStatus {
  Single,
  Married,
  Divorced,
}

#[derive(Debug)]

pub enum EducationLevel {
  HighSchool,
  Bachelor,
  Master,
  PhD,
}

impl MaritalStatus {
  pub fn to_string(&self) -> &'static str {
    match self {
      MaritalStatus::Single => "Single",
      MaritalStatus::Married => "Married",
      MaritalStatus::Divorced => "Divorced",
    }
  }

  pub fn from_str(value: &str) -> Option<Self> {
    match value {
      "Single" => Some(MaritalStatus::Single),
      "Married" => Some(MaritalStatus::Married),
      "Divorced" => Some(MaritalStatus::Divorced),
      _ => None,
    }
  }
}

impl EducationLevel {
  pub fn to_string(&self) -> &'static str {
    match self {
      EducationLevel::HighSchool => "HighSchool",
      EducationLevel::Bachelor => "Bachelor",
      EducationLevel::Master => "Master",
      EducationLevel::PhD => "PhD",
    }
  }

  pub fn from_string(value: &str) -> Option<Self> {
    match value {
      "HighSchool" => Some(EducationLevel::HighSchool),
      "Bachelor" => Some(EducationLevel::Bachelor),
      "Master" => Some(EducationLevel::Master),
      "PhD" => Some(EducationLevel::PhD),
      _ => None,
    }
  }
}

impl Gender {
  pub fn to_string(&self) -> &'static str {
    match self {
      Gender::Male => "Male",
      Gender::Female => "Female",
      Gender::Other => "Other",
    }
  }

  pub fn from_string(value: &str) -> Option<Self> {
    match value {
      "Male" => Some(Gender::Male),
      "Female" => Some(Gender::Female),
      "Other" => Some(Gender::Other),
      _ => None,
    }
  }
}
