// @generated automatically by Diesel CLI.

diesel::table! {
    auth_token (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        uid -> Varchar,
        token_type -> Integer,
        iat -> Integer,
        exp -> Integer,
        is_revoke -> Nullable<Bool>,
    }
}

diesel::table! {
    product (id) {
        #[max_length = 255]
        id -> Varchar,
        name -> Text,
        description -> Nullable<Text>,
        price -> Nullable<Double>,
        quantity -> Nullable<Double>,
        #[max_length = 255]
        type_id -> Nullable<Varchar>,
        recommend -> Nullable<Bool>,
        active -> Nullable<Bool>,
        #[max_length = 255]
        image_id -> Nullable<Varchar>,
        #[max_length = 255]
        created_by -> Varchar,
        #[max_length = 255]
        updated_by -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    product_type (id) {
        #[max_length = 255]
        id -> Varchar,
        name -> Text,
        active -> Nullable<Bool>,
        #[max_length = 255]
        image_id -> Nullable<Varchar>,
        #[max_length = 255]
        created_by -> Varchar,
        #[max_length = 255]
        updated_by -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    sale_order (id) {
        id -> Integer,
        sale_order_name -> Longtext,
        status -> Nullable<Integer>,
        total -> Nullable<Double>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    sale_order_line (id) {
        id -> Integer,
        #[max_length = 255]
        product_id -> Varchar,
        sale_order_id -> Integer,
        quantity -> Nullable<Integer>,
        total -> Nullable<Double>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    system_parameter (id) {
        id -> Integer,
        key_name -> Longtext,
        key_value -> Longtext,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    uploader (id) {
        #[max_length = 255]
        id -> Varchar,
        body -> Longtext,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        username -> Blob,
        email -> Blob,
        hashed_password -> Blob,
        active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::joinable!(auth_token -> users (uid));
diesel::joinable!(product -> product_type (type_id));
diesel::joinable!(sale_order_line -> product (product_id));
diesel::joinable!(sale_order_line -> sale_order (sale_order_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_token,
    product,
    product_type,
    sale_order,
    sale_order_line,
    system_parameter,
    uploader,
    users,
);
