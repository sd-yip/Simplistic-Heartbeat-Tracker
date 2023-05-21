// @generated automatically by Diesel CLI.

diesel::table! {
    heartbeat (id) {
        id -> Uuid,
        source -> Varchar,
        expiry -> Timestamptz,
    }
}
