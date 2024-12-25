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

diesel::allow_tables_to_appear_in_same_query!(
    auth_token,
    users,
);
