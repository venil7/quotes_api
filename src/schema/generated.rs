table! {
    holdings (id) {
        id -> Int4,
        user_id -> Int4,
        ticker -> Varchar,
        amount -> Float8,
        created -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        password -> Varchar,
        verified -> Bool,
        created -> Timestamp,
    }
}

joinable!(holdings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    holdings,
    users,
);
