use first_rust_project::front_of_house::hosting;

fn main() {
    let waiter_say: hosting::seat_at_table();
    assert_eq!(waiter_say, String::from("Sit down please"));
}