struct Rect{
    width: u32, 
    height: u32
}

struct NoShape;

impl Rect{
    fn area(&self) -> u32{
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32{
        return 2 * (self.width + self.height);
    }
}

impl NoShape{
    fn area(&self) -> u32{
        return 0;
    }    
}

fn main(){
    let rect = Rect{
        width: 30, 
        height: 50
    };

    println!("The area of the rectangle is: {}", rect.area());
    println!("The perimeter of the rectangle is: {}", rect.perimeter());

    let no_shape = NoShape;
    print!("{:?}", no_shape.area());
}