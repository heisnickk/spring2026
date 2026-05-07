const FREEZING_POINT: f64 = 32.0;

fn farenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * 5.0 / 9.0

}

/*
fn celsius_to_farenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT
}
    */

//Commented the celsius to farenheit function since it wasnt being used.

fn main() {
    
    let mut temp_f: f64 =32.0;

    let temp_c = farenheit_to_celsius(temp_f);
    println!("{}°F is {:.2}°C", temp_f, temp_c);
/*
    let back_to_f = celsius_to_farenheit(temp_c);
    println!("{:.2}°C is {:.2}°F", temp_c, back_to_f);
*/

    for _ in 0..5 {
        temp_f += 1.0;
        let converted = farenheit_to_celsius(temp_f);
        println!("{}°F is {:.2}°C", temp_f, converted);
    }

}