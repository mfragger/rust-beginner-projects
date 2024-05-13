use std::f64::consts::PI;

pub fn area_calculate_radius_diameter(number : u32) -> String {
    let mut radius_diameter = (0f64, 0f64);
    radius_diameter.0 = (number as f64 / PI).sqrt();

    radius_diameter.1 = 2f64 * radius_diameter.0;

    format!("Radius is: {}, Diameter is: {}", radius_diameter.0, radius_diameter.1)

}

pub fn radius_calculate_area_diameter(number : u32) -> String {
    let mut area_diameter = (0f64, 0f64);
    area_diameter.0 = PI * (number as f64).powi(2);

    area_diameter.1 = 2f64 * number as f64;


    format!("Area is: {}, Diameter is: {}", area_diameter.0, area_diameter.1)
}

pub fn diameter_calculate_area_radius(number : u32) -> String {
    let mut area_radius = (0f64, 0f64);
    area_radius.0 = PI * (number as f64 / 2f64).powi(2);

    area_radius.1 = number as f64 / 2f64;

    format!("Radius is: {}, Diameter is: {}", area_radius.0, area_radius.1)
}

pub fn execute() {
    println!("Please make a choice: (1) Area, (2) Radius, (3) Diameter");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to readline");
    let choice : u32 = input.trim().parse().expect("Please type a number");

    println!("Please put a number");
    input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to readline");
    let number : u32 = input.trim().parse().expect("Please type a number");

    println!( "{}", match choice {
        1 => area_calculate_radius_diameter(number),
        2 => radius_calculate_area_diameter(number),
        3 => diameter_calculate_area_radius(number),
        _ => format!("No more"),
    });
}