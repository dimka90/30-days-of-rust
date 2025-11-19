use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

trait Repository<T> {
    fn create(&mut self, item: T) -> Result<(), String>;
    fn read(&self, id: &str) -> Option<&T>;
    fn update(&mut self, id: &str, item: T) -> Result<(), String>;
    fn delete(&mut self, id: &str) -> Result<T, String>;
    fn list_all(&self) -> Vec<&T>;
}

// T must have an ID
trait HasId {
    fn id(&self) -> String;
}

struct InMemoryRepo<T> {
database: HashMap<String, T>
}

impl<T: HasId + Clone> Repository<T> for InMemoryRepo<T> {
  fn create(&mut self, item: T) -> Result<(), String>{

  }
    fn read(&self, id: &str) -> Option<&T>
    {

    }
    fn update(&mut self, id: &str, item: T) -> Result<(), String>
    {

    }
    fn delete(&mut self, id: &str) -> Result<T, String>
    {

    }
    fn list_all(&self) -> Vec<&T>{

    }

}
