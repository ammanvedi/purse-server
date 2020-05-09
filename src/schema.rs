table! {
    account_holder (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
        first_name -> Text,
        last_name -> Text,
        access_token -> Text,
    }
}

table! {
    account_transactions (id) {
        id -> Text,
        amount -> Float4,
        utc_timestamp -> Int4,
        iso_currency -> Nullable<Text>,
        name -> Nullable<Text>,
        category -> Nullable<Text>,
        pending -> Int4,
        bank_account_id -> Text,
        pending_transaction_id -> Nullable<Text>,
        transfer_transaction_id -> Nullable<Text>,
    }
}

table! {
    bank_account (id) {
        id -> Text,
        balance_current -> Float4,
        balance_available -> Float4,
        account_name -> Text,
        account_type -> Text,
        iso_currency -> Text,
        account_holder_id -> Int4,
    }
}

table! {
    categories (id) {
        id -> Text,
        description -> Text,
    }
}

table! {
    expense (id) {
        paused -> Int4,
        id -> Int4,
        frequency_type -> Text,
        frequency_date -> Nullable<Int4>,
        name -> Text,
        description -> Nullable<Text>,
        utc_timestamp_created -> Int4,
        price_bound_lower -> Float8,
        price_bound_upper -> Float8,
    }
}

table! {
    expense_transaction (expense_id, transaction_id) {
        expense_id -> Int4,
        transaction_id -> Text,
    }
}

joinable!(account_transactions -> bank_account (bank_account_id));
joinable!(account_transactions -> categories (category));
joinable!(bank_account -> account_holder (account_holder_id));
joinable!(expense_transaction -> account_transactions (transaction_id));
joinable!(expense_transaction -> expense (expense_id));

allow_tables_to_appear_in_same_query!(
    account_holder,
    account_transactions,
    bank_account,
    categories,
    expense,
    expense_transaction,
);
