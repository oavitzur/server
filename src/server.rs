
use std::net;

pub struct Server
{
    m_address: String,

}

impl Server
{
    pub fn new(addr:String) -> Self
    {
        let a = Self
        {
            m_address: addr
        };
        a
    }
    pub fn run(self)
    {
        let listener = net::TcpListener::bind(&self.m_address).unwrap();
        
        loop
        {
            let connection = listener.accept();
            if connection.is_err()
            {
                continue;
            }
        }        
    }
}