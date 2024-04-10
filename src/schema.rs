// @generated automatically by Diesel CLI.

diesel::table! {
    configuration_settings (id) {
        id -> Int4,
        section -> Text,
        name -> Text,
        value -> Text,
    }
}
