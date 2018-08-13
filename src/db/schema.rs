table! {
    use diesel::sql_types::*;
    use diesel::pg::types::sql_types::*;
    use db::models::Endcon;

    banned_ips (id) {
        id -> Int4,
        ip -> Inet,
        user_id -> Nullable<Int4>,
        admin_responsible -> Int4,
        reason -> Nullable<Text>,
        datetime_banned -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel::pg::types::sql_types::*;
    use db::models::Endcon;

    games (id) {
        id -> Int4,
        name -> Text,
        players -> Array<Int4>,
        owner -> Int4,
        variant -> Int2,
        timed -> Bool,
        seed -> Text,
        score -> Int4,
        endcon -> Endcon,
        action -> Jsonb,
        datetime_created -> Timestamp,
        datetime_started -> Timestamp,
        datetime_finished -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel::pg::types::sql_types::*;
    use db::models::Endcon;

    timed_games (id) {
        id -> Int4,
        time_base -> Int4,
        time_per_turn -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel::pg::types::sql_types::*;
    use db::models::Endcon;

    users (id) {
        id -> Int4,
        name -> Text,
        pw -> Bytea,
        salt -> Bytea,
        last_ip -> Nullable<Inet>,
        admin -> Int2,
        datetime_created -> Timestamp,
        datetime_last_login -> Timestamp,
    }
}

joinable!(games -> users (owner));
joinable!(timed_games -> games (id));

allow_tables_to_appear_in_same_query!(
    banned_ips,
    games,
    timed_games,
    users,
);
