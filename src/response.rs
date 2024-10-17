use serde::Serialize;
use crate::model::{Budget, Category};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String
}

#[derive(Serialize)]
pub struct GeneralResponse{
    pub status: String,
    pub message: String
}

#[derive(Serialize)]
pub struct CategoryListResponse {
    pub status: String,
    pub size: usize,
    pub categories: Vec<Category>,
}

#[derive(Serialize)]
pub struct BudgetListResponse {
    pub status: String,
    pub size: usize,
    pub budgets: Vec<Budget>,
}

#[derive(Serialize)]
pub struct BudgetResponse {
    pub status: String,
    pub data: Budget
}

#[derive(Serialize)]
pub struct BudgetAmountResponse {
    pub status: String,
    pub amount: f64
}
