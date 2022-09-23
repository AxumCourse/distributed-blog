use blog_proto::ListCategoryRequest;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CateListFilter {
    pub is_del: Option<String>,
    pub name: Option<String>,
    pub msg: Option<String>,
}

impl Into<ListCategoryRequest> for CateListFilter {
    fn into(self) -> ListCategoryRequest {
        ListCategoryRequest {
            name: self.name,
            is_del: match self.is_del {
                Some(s) => {
                    if s.is_empty() {
                        None
                    } else {
                        Some(&s == "true")
                    }
                }
                None => None,
            },
        }
    }
}
