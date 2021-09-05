

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut reversed = 0;
    let mut m_x = x;
    while m_x > reversed {
        reversed = reversed * 10 + m_x % 10;
        m_x /= 10;
    }

    m_x == reversed || m_x == reversed/10
}



fn main() 
{
   println!("{}",is_palindrome(1001))
}