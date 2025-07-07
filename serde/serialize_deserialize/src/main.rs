use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct OpenAIResponse{
    role: String,
    response: String
}

fn main() {
    let person = OpenAIResponse{
        role: "ai".to_string(),
        response: "Faith does not requires feeling".to_string(),
    };
    let person_to_json = serde_json::to_string(&person).unwrap();
println!("Person:{:?}", person);
println!("Json Person : {:?}", person_to_json);
let person_to_struct: OpenAIResponse = serde_json::from_str(&person_to_json).unwrap();
println!("Back to native type: {:?}", person_to_struct);
}
