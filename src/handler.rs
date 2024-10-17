use crate::{db, model::{Budget, BudgetRequest, Category, NewCategoryRequest}, response::{BudgetAmountResponse, BudgetListResponse, BudgetResponse, CategoryListResponse, ErrorResponse, GeneralResponse}};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use rust_decimal::Decimal;
use tokio_postgres::Client;
use std::sync::Arc;
use rust_decimal::prelude::*;

pub async fn get_category_list(Extension(client): Extension<Arc<Client>>) -> impl IntoResponse {
 match db::select_categories(&client).await {
        Ok(rows) => {
            let categories: Vec<Category> = rows.into_iter().map(|row| {
                Category::new (row.get("id"),row.get("category"))
            }).collect();

            let response = CategoryListResponse {
                status: "success".to_string(),
                size: categories.len(),
                categories,
            };

            Ok((StatusCode::OK, Json(response)))

        }
        Err(_e) => {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "fail to fetch data".to_string()
            };
    
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn create_category(Extension(client): Extension<Arc<Client>>, Json(body): Json<NewCategoryRequest>) -> impl IntoResponse {
    match db::insert_category(&client, &body.category).await {
        Ok(id) => {
            let response = GeneralResponse {
                status: "success".to_string(),
                message: format!("New record inserted. Id: {}", id)
            };
    
            Ok((StatusCode::OK, Json(response)))
        }
        Err(_e) => {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "fail to insert data".to_string()
            };
    
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn get_budget_list(Extension(client): Extension<Arc<Client>>) -> impl IntoResponse {
    match db::select_budgets(&client).await {
        Ok(rows) => {
            let budgets: Vec<Budget> = rows.into_iter().map(|row| {
                let float_price =  row.get::<_, Decimal>("price").to_f64().unwrap();

                Budget::new(row.get("id"), float_price, row.get("categoryId"), row.get("title"))
            }).collect();

            let response = BudgetListResponse {
                status: "success".to_string(),
                size: budgets.len(),
                budgets,
            };

            Ok((StatusCode::OK, Json(response)))
        }
        Err(_e) => {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "fail to fetch data".to_string()
            };
    
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn get_single_budget(Extension(client): Extension<Arc<Client>>, Path(id): Path<i32>) -> impl IntoResponse {
    match db::select_single_budget(&client, id).await {
      Ok(row) => {
        let float_price =  row.get::<_, Decimal>("price").to_f64().unwrap();
        
        let budget = Budget {
            id: row.get("id"),
            price: float_price,
            categoryId: row.get("categoryId"),
            title: row.get("title")
        };
        
        let response = BudgetResponse {
            status: "success".to_string(),
            data: budget
        };

        Ok((StatusCode::OK, Json(response)))
      } 
      Err(_e) => {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Data no found")
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
    }
}

pub async fn create_budget(Extension(client): Extension<Arc<Client>>, Json(body):Json<BudgetRequest>) -> impl IntoResponse {
    let decimal_price = Decimal::from_f64(body.price).unwrap();

    match db::insert_budget(&client, decimal_price, body.categoryId, &body.title).await {
        Ok(id) => {
            let response = GeneralResponse {
                status: "success".to_string(),
                message: format!("New record inserted. Id: {}", id)
            };
    
            Ok((StatusCode::OK, Json(response)))
        }
        Err(_e) => {
            println!("{}",_e);
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "fail to insert data".to_string()
            };
    
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn update_budget(Extension(client): Extension<Arc<Client>>, Path(id): Path<i32>,Json(body):Json<BudgetRequest>) -> impl IntoResponse {
    let decimal_price = Decimal::from_f64(body.price).unwrap();

    match db::update_budget(&client, id, decimal_price, body.categoryId, &body.title).await {
        Ok(()) => {
            let response = GeneralResponse {
                status: "success".to_string(),
                message: format!("Record update.")
            };
    
            Ok((StatusCode::OK, Json(response)))
        }
        Err(_e) => {
            println!("{}",_e);
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "fail to update data".to_string()
            };
    
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn remove_budget(Extension(client): Extension<Arc<Client>>, Path(id): Path<i32>) -> impl IntoResponse {
    match db::delete_budget(&client, id).await {
        Ok(()) => {
            let response = GeneralResponse {
                status: "success".to_string(),
                message: format!("Record deleted.")
            };
    
            Ok((StatusCode::OK, Json(response)))
        }
        Err(_e) => {
            println!("{}",_e);
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "fail to delete data".to_string()
            };
    
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn get_amount(Extension(client): Extension<Arc<Client>>) -> impl IntoResponse {
    match db::select_sum_of_price(&client).await {
        Ok(row) => {
            let float_amount =  row.get::<_, Decimal>("sum").to_f64().unwrap();

            let response = BudgetAmountResponse {
                status: "success".to_string(),
                amount: float_amount
            };

            Ok((StatusCode::OK, Json(response)))
        }
        Err(_e) => {
            println!("{}",_e);
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: "fail to get data".to_string()
            };
    
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

