// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(name: String, order_quantity: u32, price: u32) -> Order {
        if Self::is_valid_product_name(&name) == false {
            panic!("product name can't be empty or bigger than 300");
        }
        if Self::is_valid_quantity(&order_quantity) == false {
            panic!("quantity can't be zero");
        }
        if Self::is_valid_price(&price) == false {
            panic!("price can't be zero");
        }

        Order {
            product_name: name,
            quantity: order_quantity,
            unit_price: price,
        }
    }
    // self: &Self: the fn reads self without taking ownership
    // &self.product_name: returns a reference to the title String field, the
    // caller gets a reference to title, without taking ownership
    // &String: inmutable return value string to title
    pub fn product_name(self: &Self) -> &String {
        &self.product_name
    }

    pub fn quantity(self: &Self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(self: &Self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(self: &mut Self, new_name: String) {
        if Self::is_valid_product_name(self.product_name()) {
            self.product_name = new_name
        } else {
            panic!("product name can't be empty");
        }
    }

    pub fn set_quantity(self: &mut Self, new_quantity: u32) {
        if Self::is_valid_quantity(&new_quantity) == false {
            panic!("quantity can't be zero");
        }

        self.quantity = new_quantity
    }

    pub fn set_unit_price(self: &mut Self, new_price: u32) {
        if Self::is_valid_price(&new_price) == false {
            panic!("price can't be zero");
        }

        self.unit_price = new_price
    }

    pub fn total(self: &Self) -> u32 {
        let total = &self.quantity * &self.unit_price;
        total
    }

    fn is_valid_product_name(name: &String) -> bool {
        if name.len() > 1 && name.len() < 300 {
            true
        } else {
            false
        }
    }

    fn is_valid_quantity(quantity: &u32) -> bool {
        if *quantity > 0 {
            true
        } else {
            false
        }
    }

    fn is_valid_price(price: &u32) -> bool {
        if *price > 0 {
            true
        } else {
            false
        }
    }
}
