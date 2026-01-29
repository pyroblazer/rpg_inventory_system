// RPG Inventory System

#[derive(Debug, Clone, PartialEq)]
enum ItemType {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    item_type: ItemType,
    value: u32,
}

impl Item {
    fn new(name: &str, item_type: ItemType, value: u32) -> Self {
        Item {
            name: name.to_string(),
            item_type,
            value,
        }
    }
}

#[derive(Debug)]
struct Inventory {
    items: Vec<Item>,
    max_slots: usize,
}

impl Inventory {
    fn new(max_slots: usize) -> Self {
        Inventory {
            items: Vec::new(),
            max_slots,
        }
    }

    fn add_item(&mut self, item: Item) -> Result<(), String> {
        if self.items.len() >= self.max_slots {
            Err(format!("Inventory is full! Cannot add item: {}", item.name))
        } else {
            self.items.push(item);
            Ok(())
        }
    }

    fn remove_item(&mut self, index: usize) -> Result<Item, String> {
        if index >= self.items.len() {
            Err(format!("Invalid index: {}", index))
        } else {
            Ok(self.items.remove(index))
        }
    }

    fn get_item(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    fn is_full(&self) -> bool {
        self.items.len() >= self.max_slots
    }

    fn total_value(&self) -> u32 {
        self.items.iter().map(|item| item.value).sum()
    }
}

fn main() {
    println!("=== RPG Inventory System ===\n");

    // Create inventory with max 5 slots
    let mut inventory = Inventory::new(5);
    println!("Created inventory with max {} slots\n", inventory.max_slots);

    // Add 5 different items
    println!("--- Adding 5 items ---");
    let items_to_add = vec![
        Item::new("Dragon Slayer Sword", ItemType::Weapon, 500),
        Item::new("Diamond Armor", ItemType::Armor, 300),
        Item::new("Health Potion", ItemType::Potion, 50),
        Item::new("Magic Wand", ItemType::Weapon, 250),
        Item::new("Shield of Valor", ItemType::Armor, 200),
    ];

    for item in &items_to_add {
        match inventory.add_item(item.clone()) {
            Ok(()) => println!("✓ Added: {} (Type: {:?}, Value: {})", 
                item.name, item.item_type, item.value),
            Err(e) => println!("✗ Error: {}", e),
        }
    }

    println!("\n--- Inventory Status ---");
    println!("Is full: {}", inventory.is_full());
    println!("Total items: {}", inventory.items.len());
    println!("Total value: {}\n", inventory.total_value());

    // Try to add item when inventory is full
    println!("--- Testing error handling (adding when full) ---");
    let extra_item = Item::new("Extra Item", ItemType::Potion, 100);
    match inventory.add_item(extra_item) {
        Ok(()) => println!("✓ Added extra item"),
        Err(e) => println!("✓ Error handled correctly: {}", e),
    }

    // Remove a specific item (index 2 - Health Potion)
    println!("\n--- Removing item at index 2 ---");
    match inventory.remove_item(2) {
        Ok(removed_item) => println!("✓ Removed: {} (Value: {})", 
            removed_item.name, removed_item.value),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n--- Inventory after removal ---");
    println!("Total items: {}", inventory.items.len());
    println!("Total value: {}\n", inventory.total_value());

    // Print all items
    println!("--- All Items in Inventory ---");
    println!("{:<4} {:<20} {:<10} {:<10}", "Idx", "Name", "Type", "Value");
    println!("{}", "-".repeat(50));
    
    for (index, item) in inventory.items.iter().enumerate() {
        let type_str = match item.item_type {
            ItemType::Weapon => "Weapon",
            ItemType::Armor => "Armor",
            ItemType::Potion => "Potion",
        };
        println!("{:<4} {:<20} {:<10} {:<10}", 
            index, item.name, type_str, item.value);
    }

    println!("\n--- Final Summary ---");
    println!("Total items in inventory: {}", inventory.items.len());
    println!("Total value of all items: {}", inventory.total_value());
    println!("Inventory is full: {}", inventory.is_full());

    // Demonstrate get_item
    println!("\n--- Get Item Demo ---");
    if let Some(item) = inventory.get_item(0) {
        println!("Item at index 0: {} (Type: {:?}, Value: {})", 
            item.name, item.item_type, item.value);
    }
}
