// @generated automatically by Diesel CLI.

diesel::table! {
    careers (id) {
        id -> Integer,
        class -> Text,
        career -> Text,
        career_level -> Text,
    }
}

diesel::table! {
    skills (skill_id) {
        skill_id -> Integer,
        title -> Text,
        attribute -> Text,
        is_advanced -> Nullable<Integer>,
        is_grouped -> Nullable<Integer>,
        skill_description -> Nullable<Text>,
    }
}

diesel::table! {
    skills_spec (id) {
        id -> Integer,
        title -> Text,
        skill_id -> Nullable<Integer>,
    }
}

diesel::table! {
    talents (id) {
        id -> Integer,
        title -> Text,
        maximum -> Nullable<Text>,
        tests -> Nullable<Text>,
        talent_description -> Nullable<Text>,
    }
}

diesel::joinable!(skills_spec -> skills (skill_id));

diesel::allow_tables_to_appear_in_same_query!(
    careers,
    skills,
    skills_spec,
    talents,
);
