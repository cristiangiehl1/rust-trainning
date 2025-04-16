use topics::{associated_function::TrafficLight, generics::Val, methods::TrafficLightColorEnum};

mod topics;

fn main() {
    let light = TrafficLight::new();
    println!(
        "The initial state of the traffic light is: {}",
        light.get_state()
    );

    let color = TrafficLightColorEnum::Yellow;
    println!("The color of the traffic light is: {}", color.color());

    let x = Val { val: 5 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("x: {}, y: {}", x.value(), y.value());
}
