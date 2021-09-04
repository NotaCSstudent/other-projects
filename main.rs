struct Node
{
   x : i32,
   y : i32,
   z : i32,
}

impl Node 
{
    pub fn create(mut self, a:i32,b:i32,c:i32)
    {
        self.x = a;
        self.y = b;
        self.z = c;
    }

}



fn main() 
{
    let t = Node{ x: 32, y:42 , z:89};
    //t.create(1,2,3);
    println!("{}",t.x);
    println!("Hello World")
}