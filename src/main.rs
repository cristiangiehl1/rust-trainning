// https://www.youtube.com/watch?v=BpPEoZW5IiY&t=40108s

use topics::{
    associated_function::TrafficLight,
    closures::{
        _example_closure_capturing_01, _example_closure_capturing_02, _example_closures_04,
        _example_closures_06, _example_closures_09,
    },
    debug_display::{example_debug_04, example_display_01, example_display_02},
    generics::{Array, Point2, Val},
    iterators::{_example_iterators_07, _example_iterators_08, _example_iterators_10},
    lifetimes::example_lifetime_annotation_02,
    methods::TrafficLightColorEnum,
    string::{example_4, exercise_3},
    trait_object::{return_animal, reuturn_animal_2, Bird, Duck, Swan},
    vectors::{vectors_exercise_2, vectors_exercise_4},
};

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

    let generic_point1 = Point2 { x: 5, y: 10.0 };
    let generic_point2 = Point2 { x: "Point", y: '#' };

    let mixed_associated_point = Point2::mixup_associated(&generic_point1, &generic_point2);
    println!(
        "Mixed Associated Point: x = {}, y = {}",
        mixed_associated_point.x, mixed_associated_point.y
    );

    let mixed_point = generic_point1.mixup(generic_point2);

    println!("Mixed Point: x = {}, y = {}", mixed_point.x, mixed_point.y);

    let const_generics_arr = [
        Array { data: [1, 2, 3] },
        Array { data: [5, 4, 3] },
        Array { data: [10, 20, 30] },
    ];

    println!("Const Generics Array: {:?}", const_generics_arr);

    let animal = return_animal("dog");

    println!("Animal: {:?}", animal);

    let animal2 = reuturn_animal_2("cat");

    println!("Animal2: {:?}", animal2);

    let birds: [&dyn Bird; 2] = [&Duck, &Swan];

    for bird in birds {
        bird.quack();
    }

    vectors_exercise_4();

    example_debug_04();
    example_display_01();
    example_display_02();
    example_lifetime_annotation_02();
    _example_closure_capturing_01();
    _example_closure_capturing_02();
    _example_closures_04();
    _example_closures_06();
    _example_closures_09();
    _example_iterators_07();
    _example_iterators_08();
    _example_iterators_10();
}
