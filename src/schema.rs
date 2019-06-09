table! {
    consoles (_id) {
        _id -> Integer,
        short_name -> Text,
        long_name -> Text,
    }
}

table! {
    developer_map (_id) {
        _id -> Integer,
        game_id -> Integer,
        company_id -> Integer,
    }
}

table! {
    game_companies (_id) {
        _id -> Integer,
        company_name_en -> Text,
        company_name_ja -> Nullable<Text>,
    }
}

table! {
    games (_id) {
        _id -> Integer,
        title_en -> Text,
        title_ja -> Text,
        serial_code -> Nullable<Text>,
        console_id -> Nullable<Integer>,
        link_url -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

table! {
    genre_map (_id) {
        _id -> Integer,
        game_key -> Integer,
        genre_key -> Integer,
    }
}

table! {
    genres (_id) {
        _id -> Integer,
        genre -> Text,
    }
}

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

table! {
    publisher_map (_id) {
        _id -> Integer,
        game_id -> Integer,
        company_id -> Integer,
    }
}

joinable!(developer_map -> game_companies (company_id));
joinable!(developer_map -> games (game_id));
joinable!(genre_map -> games (game_key));
joinable!(publisher_map -> games (game_id));

allow_tables_to_appear_in_same_query!(
    consoles,
    developer_map,
    game_companies,
    games,
    genre_map,
    genres,
    logs,
    posts,
    publisher_map,
);
