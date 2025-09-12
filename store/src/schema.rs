// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "website_status"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    regions (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    website_ticks (id) {
        id -> Text,
        response_time_ms -> Int4,
        region_id -> Text,
        website_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        status -> WebsiteStatus,
    }
}

diesel::table! {
    websites (id) {
        id -> Text,
        url -> Text,
        user_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(website_ticks -> regions (region_id));
diesel::joinable!(website_ticks -> websites (website_id));
diesel::joinable!(websites -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    regions,
    users,
    website_ticks,
    websites,
);
