// @generated automatically by Diesel CLI.

diesel::table! {
    temporaldata (id) {
        id -> Int4,
        timestamp -> Timestamp,
        value -> Float4,
    }
}
