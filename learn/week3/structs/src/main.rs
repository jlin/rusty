struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product{
    fn new(name: String, price: f32) -> Product {
        Product{
            // redundant to set name to name, just give it name
            //name: name,
            name,
            
            // redundant to set price to price, just give it price
            //price: price,
            price,
            in_stock: true,
        
        }
    }
    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
        // use the :: syntax on the struct name to call associated functions.
    }

     // associated function doesn't take self (of any kind) as an input/parameter
     fn get_default_sales_tax() -> f32 {
        0.1
    }
    
    fn set_price(&mut self, price:f32) {
        self.price = price
    } 

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought!");
        12345
    }

   
}

fn main() {
    println!("Hello, world!");
    
    let mut book = Product {
        name: String::from("Book"),
        price: 35000.00,
        in_stock: true,
    };
    let price = book.price;
    book.in_stock=false;

    let sales_tax = book.calculate_sales_tax();
    println!("{sales_tax}");

    book.set_price(1.00);
    book.buy();

    // not allowed to set book price because the buy method owns the book. since the method ended
    // the variable is dropped, and so is no longer available to change

    // book.set_price(5.00);

    let mut book = Product::new(String::from("newbook2"), 5000.00);

}

