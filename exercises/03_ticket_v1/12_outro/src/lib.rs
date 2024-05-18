// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
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
    // validators
    fn validate_product_name(name: &str) {
        assert!(
            !name.is_empty() && name.len() <= 300,
            "Product name must be between 1 and 300 characters"
        );
    }
    fn validate_quantity(quantity: u32) {
        assert!(quantity > 0, "Quantity must be greater than 0");
    }

    fn validate_unit_price(price: u32) {
        assert!(price > 0, "Unit price must be greater than 0");
    }

    // public API

    /// Create a new order.
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        Self::validate_product_name(&product_name);
        Self::validate_quantity(quantity);
        Self::validate_unit_price(unit_price);

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    /// Total price of the order
    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    // getters and setters
    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn set_product_name(&mut self, product_name: String) {
        Self::validate_product_name(&product_name);
        self.product_name = product_name;
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        Self::validate_quantity(quantity);
        self.quantity = quantity;
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        Self::validate_unit_price(unit_price);
        self.unit_price = unit_price;
    }
}
