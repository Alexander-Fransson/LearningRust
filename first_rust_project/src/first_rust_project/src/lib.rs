pub mod front_of_house;
pub mod back_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    // we are already in the create root of the project
    front_of_house::hosting::add_to_waitlist();
}