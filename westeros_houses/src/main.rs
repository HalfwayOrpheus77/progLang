// First, I will define a structure for Character
struct Character {
    name: String,
    house: House,
}

// Define an enumeration for a House
enum House {
    Stark,
    Lannister,
    Targaryen,
    None,
}

// Implement a method for the Character structure
impl Character {
    fn new(name: &str, house: House) -> Character {
        Character {
            name: name.to_string(),
            house,
        }
    }

    fn print(&self) {
        match self.house {
            House::Stark => println!("{} of House Stark", self.name),
            House::Lannister => println!("{} of House Lannister", self.name),
            House::Targaryen => println!("{} of House Targaryen", self.name),
            House::None => println!("{} does not belong to any house", self.name),
        }
    }
}

// Main function
fn main() {
    let jon = Character::new("Jon Snow", House::Stark);
    let cersei = Character::new("Cersei", House::Lannister);
    let daenerys = Character::new("Daenerys", House::Targaryen);
    let night_king = Character::new("Night King", House::None);

    jon.print();
    cersei.print();
    daenerys.print();
    night_king.print();
}
