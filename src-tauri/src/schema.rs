// @generated automatically by Diesel CLI.

diesel::table! {
    talents (id) {
        id -> Text,
        title -> Text,
        maximum -> Text,
        tests -> Text,
        talent_desc -> Text,
    }
}
