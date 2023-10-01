// @generated automatically by Diesel CLI.

diesel::table! {
    talents (id) {
        id -> Integer,
        title -> Text,
        maximum -> Nullable<Text>,
        tests -> Nullable<Text>,
        talent_description -> Nullable<Text>,
    }
}
