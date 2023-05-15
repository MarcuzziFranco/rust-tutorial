pub mod buy {
    #[derive(Debug)]
    pub struct Item {
        name: String,    // Nombre del item
        unit_price: f32, // Precio Unitario del item
        amount: f32, // Cantidad a comprar del item, es float porque pueden ser fracciones de unidades, como kilos
    }

    pub fn add_item(items_buy: &mut Vec<Item>, item: Item) {
        // Agregara un item a un vector con todos los items de la compra
        items_buy.push(item);
    }

    pub fn remove_item(items_buy: &mut Vec<Item>, index: usize) {
        // Quitara un item del array a partir de un indice
        items_buy.remove(index);
    }

    pub fn show_items(items_buy: &Vec<Item>) {
        for (index, item) in items_buy.iter().enumerate() {
            let sub_total = item.amount * item.unit_price;
            println!(
                "[{}]. {} - Amount: {} - Price U: ${} - Subtotal: ${}",
                index, item.name, item.amount, item.unit_price, sub_total
            );
        }
    }

    pub fn total_buy(items_buy: &Vec<Item>) -> f32 {
        // Devolvera el total a pagar de todos los items de la compra
        let mut total_buy: f32 = 0.0;
        for item in items_buy {
            total_buy = total_buy + (item.amount * item.unit_price);
        }

        let y = 10i32.pow(2) as f32;
        total_buy = (total_buy * y).round() / y;
        return total_buy;
    }
}
