
fn main() {
    
    //println!("Hello, world!");


    let vec = vec![1,2,3,4,5,6,7,8,9,10];

    println!("Show array : {:?}", vec);

    //调用求和函数
    let sum = cal_sum(&vec);

    match sum{
        Some(x) => {
            println!("Sum is {}", x); 
        }
        None => {
            println!("ERR: Overflow");
        }

    }
    

    let vec = vec![4294967292,5];

    println!("Show array : {:?}", vec);

    //调用求和函数
    let sum = cal_sum(&vec);

    match sum{
        Some(x) => {
            println!("Sum is {}", x); 
        }
        None => {
            println!("None: Calculation result is overflow,return None.");
        }

    }


}


fn cal_sum(a:&Vec<u32>)->Option<u32>{

    let mut x:u32 = 0;

    for i in a{
        //数列求和
        
        if u64::from(x+i) >=2u64.pow(8) {
            
            return None;

        } else { 

            x += i;
        
        }
                

    }
    //完成计算，返回求和结果
    Some(x)

}

