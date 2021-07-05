
mod server;
mod http;

fn main() 
{
/*  
    let str1 = String::from("127.0.0.1:8080");
    let str_slc: &str = &str1[10..]; 

    let request = http::Request { 
            m_path: "a".to_string(),
            m_query: Some("a".to_string()), 
            m_method: http::HttpMethod::GET("".to_string()) 
    };
    //dbg!(&request);

    println!("str {}", str_slc);

    dbg!(&str1);

    let str_slice = &str1[10..];
    dbg!(str_slice);
*/
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();

    // println!("Hello, world!" );
}


