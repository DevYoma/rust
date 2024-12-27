#[warn(unused_variables)]

enum Direction{
    North, 
    East, 
    South, 
    West
}

enum Shape{
    Circle(f64), 
    Square(f64), 
    Rectangle(f64, f64)
}

fn calculate_area(shape: Shape) -> f64{
    // calculates area of shape
    let ans = match shape{
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius, 
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height
    };

    return ans;
}

fn main(){
    let my_direction = Direction::North;
    move_around(my_direction);

    // println!("Direction {}, New Direction {}", my_direction)

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0); 

    let area = calculate_area(circle);
    println!("The area of the circle is {}", area);

}

fn move_around(direction: Direction){
    // implement logic ot move around
}