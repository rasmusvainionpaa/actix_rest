table! {
    note (id) {
        id -> Uuid,
        title -> Text,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}