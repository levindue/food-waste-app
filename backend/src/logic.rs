use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::Read;


#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    name: String,
    food_cannot_eat: Vec<String>,
}

impl Person {
    pub fn add_food(&mut self, food: &str) {
        self.food_cannot_eat.push(food.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleList {
    pub people: Vec<Person>,
}

impl PeopleList {
    pub fn new() -> Self {
        PeopleList { people: Vec::new() }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut content = String::new();
        let mut file = File::open(filename)?;
        file.read_to_string(&mut content)?;

        let people_list: PeopleList = serde_json::from_str(&content)?;
        Ok(people_list)
    }
}
