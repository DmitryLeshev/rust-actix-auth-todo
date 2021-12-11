use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub limit: i64,
    pub current_page: i64,
    pub total_pages: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_page: Option<String>,
}

impl Pagination {
    pub fn new(number_items: i64, limit: i64, page: i64) -> Self {
        let total_pages = (number_items as f64 / limit as f64).ceil() as i64;
        Self {
            total_pages,
            limit,
            current_page: page,
            next_page: None,
            prev_page: None,
        }
    }
    pub fn create_links(&self, base_link: String) -> Self {
        let next_page = if self.current_page < self.total_pages {
            Some(format!(
                "{}page={}&limit={}",
                base_link,
                self.current_page + 1,
                self.limit
            ))
        } else {
            None
        };
        let prev_page = if self.current_page != 1 {
            Some(format!(
                "{}page={}&limit={}",
                base_link,
                self.current_page - 1,
                self.limit
            ))
        } else {
            None
        };

        Self {
            next_page,
            prev_page,
            ..self.clone()
        }
    }
}
