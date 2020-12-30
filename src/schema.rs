table! {
    rustacean_note (id) {
        id -> Integer,
        rustacean_id -> Integer,
        text -> Text,
        created_at -> Timestamp,
    }
}

table! {
    rustaceans (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

joinable!(rustacean_note -> rustaceans (rustacean_id));

allow_tables_to_appear_in_same_query!(
    rustacean_note,
    rustaceans,
);
