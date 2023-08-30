use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub food: Vec<Food>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Food {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub food: HashMap<u32, Food>,
    pub people: HashMap<u32, Person>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            food: HashMap::new(),
            people: HashMap::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.insert(person.id, person);
    }

    pub fn remove_person(&mut self, person_id: u32) {
        self.people.remove(&person_id);
    }

    pub fn list_people(&self) -> Vec<&Person> {
        self.people.values().collect()
    }

    pub fn add_food(&mut self, food: Food) {
        self.food.insert(food.id, food);
    }

    pub fn remove_food(&mut self, food_id: u32) {
        self.food.remove(&food_id);
    }

    pub fn list_food(&self, person: &Person) -> Vec<&Food> {
        person
            .food
            .iter()
            .map(|food| self.food.get(&food.id).unwrap())
            .collect()
    }

    pub fn list_all_food(&self) -> Vec<&Food> {
        self.food.values().collect()
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        let serialized = serde_json::to_string(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file(filename: &str) -> Result<Self, io::Error> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let manager: Manager = serde_json::from_str(&contents)?;
        Ok(manager)
    }
}
