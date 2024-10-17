use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Category{
    pub id: Option<i32>,
    pub category: String
}

impl Category {
    pub fn new(id: i32, category: String) -> Category {
        Category {
            id: Some(id), category
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NewCategoryRequest {
    pub category: String
}
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Budget{
    pub id: i32,
    pub price: f64,
    pub categoryId: i32,
    pub title: Option<String>
}

impl Budget {
    pub fn new(id: i32, price: f64, category_id: i32, title: String) -> Budget {
        Budget {
            id, price, categoryId: category_id, title: Some(title)
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct BudgetRequest {
    pub price: f64,
    pub categoryId: i32,
    pub title: String
}
