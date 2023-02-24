// @generated automatically by Diesel CLI.

diesel::table! {
    todo_shares (id) {
        id -> Integer,
        todo_id -> Integer,
        user_id -> Integer,
        is_accept -> Bool,
        is_deleted -> Bool,
        created_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    todos (id) {
        id -> Integer,
        user_id -> Integer,
        status -> Bool,
        title -> Varchar,
        description -> Nullable<Varchar>,
        target_date -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        name -> Varchar,
        password -> Varchar,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todo_shares,
    todos,
    users,
);
