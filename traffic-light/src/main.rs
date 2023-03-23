use std::io;


fn main() {
    

     let mut light_color = String::new();

     //Input the color to be checked from screen.
     println!("Please input the color you want to check:");
     io::stdin()
         .read_line(&mut light_color)
         .expect("Failed to read line");


    //Transfer the type string input to the TrafficLight enum
    let light_color = light_color.trim();

    let traffic_light = check_light_color(&light_color);//TrafficLight enum

    
    //Use the return_time method to check time.
    let time : u8 = traffic_light.return_time();

    //Output the result
    if time>0 {
        println!("{time}");
    } else {
        println!("ERR:You have checked a wrong color.");
    }


    
    

}

enum TrafficLight{
    Red,
    Yellow,
    Green,
    Invaild,
}




impl ReturnTime for TrafficLight{

    fn return_time(&self) -> u8 {

        match &self {
            TrafficLight::Red => 50,
            TrafficLight::Green => 80,
            TrafficLight::Yellow => 10,
            TrafficLight::Invaild => 0,
        }

    }

}



pub trait ReturnTime{
    fn return_time(&self) -> u8;
}



fn check_light_color(color:&str) -> TrafficLight{

    if color == "red"{

        let light = TrafficLight::Red;
        return light;
    }

    

    else if color == "yellow" {

        let light = TrafficLight::Yellow;
        return light;

    }
            
       

    else if color== "green" {

        let light = TrafficLight::Green;
        return light;

    }
 
    else {

        let light = TrafficLight::Invaild;
        return light;

    }
}
    