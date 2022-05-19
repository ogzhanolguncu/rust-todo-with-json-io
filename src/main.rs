mod to_do;

use to_do::structs::traits::create::Create;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let todo_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");

    match todo_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title
        ),
    }
}
