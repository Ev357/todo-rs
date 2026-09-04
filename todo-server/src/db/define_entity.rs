#[macro_export]
macro_rules! define_entity {
    (
        $create_name:ident, $patch_name:ident, $name:ident {
            $id_field:ident : $id_type:ty,
            $($field:ident : $ftype:ty),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, sqlx::prelude::FromRow, serde::Serialize)]
        pub struct $name {
            pub $id_field: $id_type,
            $(pub $field: $ftype,)*
        }

        #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
        pub struct $create_name {
            $(pub $field: $ftype,)*
        }

        #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
        pub struct $patch_name {
            $(pub $field: Option<$ftype>,)*
        }
    };
}
