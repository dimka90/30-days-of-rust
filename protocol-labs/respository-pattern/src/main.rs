use std::collections::HashMap;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let person1 = User{
        id: "1234".to_string(),
        name: "James".to_string()
    };

    let mut memory = InMemoryRepo::<User>::new();
    let result = memory.create(person1);

    println!("{memory:?}")
}


#[derive(Clone, Debug)]
struct User {
    id: String,
    name: String,
}

impl HasId for User {
    fn id(&self) -> String {
        self.id.clone()
    }
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

#[derive(Debug)]
struct InMemoryRepo<T> {
    database: HashMap<String, T>,
}

impl<T> InMemoryRepo<T> {
    fn new() -> Self {
        Self { database: HashMap::new() }
    }
}
impl<T: HasId + Clone> Repository<T> for InMemoryRepo<T> {
  fn create(&mut self, item: T) -> Result<(), String>{
    let id = item.id();
    self.database.insert(id, item);
    Ok(())

  }
    fn read(&self, id: &str) -> Option<&T>
    {
    self.database.get(id)

    }
    fn update(&mut self, id: &str, item: T) -> Result<(), String>
    {
        if self.database.get(id).is_none(){
            return Err("User not found".to_string());
        };
        self.database.insert(id.to_string(), item);
        Ok(())

    }
    fn delete(&mut self, id: &str) -> Result<T, String>
    {
        if self.database.get(id).is_none(){
            return  Err("User not found".to_string())
        };

        let user_to_delete = self.database.get(id).unwrap().clone();
        self.database.remove(&id.to_string());
        Ok(user_to_delete)
    }
    fn list_all(&self) -> Vec<&T>{
        
        let mut all_users = Vec::new();
        for (key, user) in &self.database{
            all_users.push(user);
        }
        all_users
    }

}
