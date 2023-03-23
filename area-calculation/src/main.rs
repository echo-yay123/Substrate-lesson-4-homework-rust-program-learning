
fn main() {
    
    let x = Rectangle{
        
        width: 10.0,
        length: 10.0,

    };

    print_area_cal(&x);


    let x = Triangle{
        
        height: 10.0,
        length: 10.0,

    };
   
    print_area_cal(&x);


    let x = Circle{
        
        radius: 10.0,

    };

    print_area_cal(&x);



}



#[derive(Debug)]
struct Rectangle{

    width : f32,
    length : f32,

}


#[derive(Debug)]
struct Triangle{

    length : f32,
    height : f32,

}
#[derive(Debug)]
struct Circle{

    radius : f32,
}


impl AreaCal for Rectangle{
    
    fn area_cal(&self) -> f32 {
      
     self.width * self.length
     

    }
}



impl AreaCal for Triangle{
    fn area_cal(&self) -> f32 {

      self.height * self.length / 2.0

    }
}


impl AreaCal for Circle{
    fn area_cal(&self) -> f32 {

      self.radius * self.radius * 3.14

    }
}


pub trait AreaCal{

    fn area_cal(&self) -> f32;  
}


pub fn print_area_cal<T:AreaCal + std::fmt::Debug>(item:&T) {

    println!("Shape parameter is {:?}",item);
    
    println!("The area is {}",item.area_cal());

}
