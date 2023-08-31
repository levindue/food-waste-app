use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub food: Vec<Food>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Food {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub people: HashMap<u32, Person>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            people: HashMap::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.insert(person.id, person);
    }

    pub fn remove_person(&mut self, person_id: u32) -> Option<Person> {
        self.people.remove(&person_id)
    }

    pub fn list_people(&self) -> Vec<&Person> {
        self.people.values().collect()
    }

    pub fn add_food(&mut self, food: Food, person_id: u32) -> Result<(), String> {
        if let Some(person) = self.people.get_mut(&person_id) {
            person.food.push(food);
            Ok(())
        } else {
            Err("Person not found".to_string())
        }
    }

    pub fn remove_food(&mut self, food_id: u32, person_id: u32) -> Result<(), String> {
        if let Some(person) = self.people.get_mut(&person_id) {
            person.food.retain(|food| food.id != food_id);
            Ok(())
        } else {
            Err("Person not found".to_string())
        }
    }

    pub fn list_food(&self, person_id: u32) -> Result<&[Food], String> {
        if let Some(person) = self.people.get(&person_id) {
            Ok(&person.food)
        } else {
            Err("Person not found".to_string())
        }
    }

    pub fn list_all_food(&self) -> Vec<&Food> {
        self.people
            .values()
            .flat_map(|person| &person.food)
            .collect()
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let serialized = serde_json::to_string(self)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file(filename: &str) -> Result<Self, io::Error> {
        let mut contents = String::new();
        File::open(filename)?.read_to_string(&mut contents)?;
        let manager: Manager = serde_json::from_str(&contents)?;
        Ok(manager)
    }
}
