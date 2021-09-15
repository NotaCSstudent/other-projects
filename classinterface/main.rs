
struct X 
{
    x : i128,
    y : i128,
    z : i128,
}

impl X {
    fn new(a:i128,b:i128,c:i128) -> X {
        X{x:a,y:b,z:c}
    }
}


struct Y
{
    x : i128,
    y : i128,
    z : i128,
}


impl Y {
    fn new(a:i128,b:i128,c:i128) -> Y {
        Y{x:a,y:b,z:c}
    }
    
}

fn main()
{
    let h = X
    {
        x : 1,
        y : 2,
        z : 3,
    };
    let t = X::new(1, 2, 3);
    println!("{}",h.x);
    println!("{}",h.y);
    println!("{}",h.z);
    println!("{}{}{}",t.x,t.y,t.z);
}