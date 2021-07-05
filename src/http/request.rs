
use super::HttpMethod;

pub struct Request
{
    pub m_path: String,
    pub m_query: Option<String>,
    pub m_method: HttpMethod,
}
