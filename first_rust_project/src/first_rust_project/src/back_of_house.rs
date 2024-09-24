fn fix_incorrect_order() {
    cook_order();
    // you use super to go one level up in the hierarchy like ../
    super::front_of_house::serving::serve_order();   
}

