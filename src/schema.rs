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
        price -> Nullable<Decimal>,
        quantity -> Nullable<Decimal>,
        #[max_length = 255]
        type_id -> Nullable<Varchar>,
        recommend -> Nullable<Bool>,
        active -> Nullable<Bool>,
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
        created_by -> Varchar,
        #[max_length = 255]
        updated_by -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    uploader (id) {
        #[max_length = 255]
        id -> Varchar,
        url -> Text,
        #[sql_name = "ref"]
        ref_ -> Text,
        #[max_length = 255]
        ref_id -> Varchar,
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

diesel::allow_tables_to_appear_in_same_query!(
    auth_token,
    product,
    product_type,
    uploader,
    users,
);
