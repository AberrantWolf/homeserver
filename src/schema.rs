table! {
    logs (id) {
        id -> Integer,
        msg -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    logs,
    posts,
);
