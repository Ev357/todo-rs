#[macro_export]
macro_rules! define_entity {
    (
        $create_name:ident, $patch_name:ident, $name:ident {
            $id_field:ident : $id_type:ty,
            $($field:ident : $ftype:ty),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "sqlx", derive(sqlx::prelude::FromRow))]
        pub struct $name {
            pub $id_field: $id_type,
            $(pub $field: $ftype,)*
        }

        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct $create_name {
            $(pub $field: $ftype,)*
        }

        #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct $patch_name {
            $(pub $field: Option<$ftype>,)*
        }
    };
}
