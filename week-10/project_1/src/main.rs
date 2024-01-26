struct Laptop {
    quantity_hp:u32,
    quantity_ibm:u32,
    quantity_toshiba:u32,
    quantity_dell:u32,
    cost_hp:u32,
    cost_ibm:u32,
    cost_toshiba:u32,
    cost_dell:u32,
}

impl Laptop {
    fn sum(&self)->u32 {
        (self.quantity_hp * self.cost_hp) + (self.quantity_ibm * self.cost_ibm) + (self.quantity_toshiba * self.cost_toshiba) + 
        (self.quantity_dell * self.cost_dell)
    }
}

fn main() {
    let total_cost = Laptop {
        quantity_hp:3,
        quantity_ibm:3,
        quantity_toshiba:3,
        quantity_dell:3,
        cost_hp:650000,
        cost_ibm:755000,
        cost_toshiba:550000,
        cost_dell:850000,
    };

    println!("total cost is {}",total_cost.sum());
}