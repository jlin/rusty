struct Product {
    name: String,
    price: f32,
    in_stock: bool
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

    let sales_tax = calculate_sales_tax(&book);
    println!("{sales_tax}")
}

fn calculate_sales_tax(product: &Product) -> f32 {
    product.price * 0.10
}