#[derive(Queryable)]
pub struct AccountHolder {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub access_token: String,
}

#[derive(Queryable)]
pub struct Category {
    pub id: i32,
    pub description: String,
}

#[derive(Queryable)]
pub struct BankAccount {
    pub id: i32,
    pub balance_current: f32,
    pub balance_available: f64,
    pub account_name: String,
    pub account_type: String,
    pub iso_currency: String,
    pub account_holder_id: i32,
}

#[derive(Queryable)]
pub struct AccountTransaction {
    pub id: i32,
    pub amount: f32,
    pub utc_timestamp: i32,
    pub iso_currency: String,
    pub name: String,
    pub category: String,
    pub pending: i32,
    pub bank_account_id: String,
    pub pending_transaction_id: String,
    pub transfer_transaction_id: String,
}

#[derive(Queryable)]
pub struct Expense {
    pub paused: i32,
    pub id: i32,
    pub frequency_type: String,
    pub frequency_date: i32,
    pub name: String,
    pub description: String,
    pub utc_timestamp_created: i32,
    pub price_bound_lower: f32,
    pub price_bound_upper: f32,
}

#[derive(Queryable)]
pub struct ExpenseTransaction {
    pub expense_id: i32,
    pub transaction_id: i32,
}
