fn main() {
    println!("Weight on Mars: {}kg", calculate_weight_on_mars(100.0));

    calculate_weight_on_mars(100.0);
}

fn calculate_weight_on_mars(_weight: f32) -> f32 {
    50.0
}


