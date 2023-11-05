trait Shape {
    fn area(&self) -> f32;
}

#[derive(Debug, Default)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

#[derive(Debug, Default)]
struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

fn get_area_of_shape(shape: Box<dyn Shape>) -> f32 {
    shape.area()
}

pub fn shape_area() {
    println!("rectangle (r) or circle (c)?");
    let mut shape_choice: String = String::from("");
    std::io::stdin().read_line(&mut shape_choice);
    shape_choice.pop();

    let shape: Box<dyn Shape>;

    if shape_choice.as_str() == "r" {
        let mut height: String = String::from("");
        let mut width: String = String::from("");

        println!("enter rectangle height:");
        std::io::stdin().read_line(&mut height);
        println!("enter rectangle width:");
        std::io::stdin().read_line(&mut width);

        width.pop();
        height.pop();

        shape = Box::new(Rectangle {
            width: width.as_str().parse().unwrap_or_default(),
            height: height.as_str().parse().unwrap_or_default(),
        });
        //dbg!(Rectangle::default());
    } else if shape_choice.as_str() == "c" {
        let mut radius: String = String::from("");

        println!("enter circle radius:");
        std::io::stdin().read_line(&mut radius);

        radius.pop();

        shape = Box::new(Circle {
            radius: radius.as_str().parse().unwrap_or_default(),
        });
    } else {
        return;
    }

    println!("area is={}", get_area_of_shape(shape));
}
