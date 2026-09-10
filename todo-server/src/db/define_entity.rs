#[macro_export]
macro_rules! define_entity {
    (
        $create_name:ident, $patch_name:ident, $name:ident {
            $id_field:ident : $id_type:ty,
            $($rest:tt)*
        }
    ) => {
        $crate::define_entity! {
            @munch
            meta: ($create_name, $patch_name, $name, $id_field, $id_type),
            input: ($($rest)*),
            all_fields: (),
            user_fields: ()
        }
    };

    (
        @munch
        meta: ($create_name:ident, $patch_name:ident, $name:ident, $id_field:ident, $id_type:ty),
        input: (#[server] $field:ident : $ftype:ty, $($rest:tt)*),
        all_fields: ($($all:tt)*),
        user_fields: ($($user:tt)*)
    ) => {
        $crate::define_entity! {
            @munch
            meta: ($create_name, $patch_name, $name, $id_field, $id_type),
            input: ($($rest)*),
            all_fields: ($($all)* $field : $ftype,),
            user_fields: ($($user)*)
        }
    };

    (
        @munch
        meta: ($create_name:ident, $patch_name:ident, $name:ident, $id_field:ident, $id_type:ty),
        input: (#[server] $field:ident : $ftype:ty),
        all_fields: ($($all:tt)*),
        user_fields: ($($user:tt)*)
    ) => {
        $crate::define_entity! {
            @munch
            meta: ($create_name, $patch_name, $name, $id_field, $id_type),
            input: (),
            all_fields: ($($all)* $field : $ftype,),
            user_fields: ($($user)*)
        }
    };

    (
        @munch
        meta: ($create_name:ident, $patch_name:ident, $name:ident, $id_field:ident, $id_type:ty),
        input: ($field:ident : $ftype:ty, $($rest:tt)*),
        all_fields: ($($all:tt)*),
        user_fields: ($($user:tt)*)
    ) => {
        $crate::define_entity! {
            @munch
            meta: ($create_name, $patch_name, $name, $id_field, $id_type),
            input: ($($rest)*),
            all_fields: ($($all)* $field : $ftype,),
            user_fields: ($($user)* $field : $ftype,)
        }
    };

    (
        @munch
        meta: ($create_name:ident, $patch_name:ident, $name:ident, $id_field:ident, $id_type:ty),
        input: ($field:ident : $ftype:ty),
        all_fields: ($($all:tt)*),
        user_fields: ($($user:tt)*)
    ) => {
        $crate::define_entity! {
            @munch
            meta: ($create_name, $patch_name, $name, $id_field, $id_type),
            input: (),
            all_fields: ($($all)* $field : $ftype,),
            user_fields: ($($user)* $field : $ftype,)
        }
    };

    (
        @munch
        meta: ($create_name:ident, $patch_name:ident, $name:ident, $id_field:ident, $id_type:ty),
        input: (),
        all_fields: ($($all_field:ident : $all_type:ty,)*),
        user_fields: ($($user_field:ident : $user_type:ty,)*)
    ) => {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "sqlx", derive(sqlx::prelude::FromRow))]
        pub struct $name {
            pub $id_field: $id_type,
            $(pub $all_field: $all_type,)*
        }

        #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct $create_name {
            $(
                #[serde(default)]
                pub $user_field: $user_type,
            )*
        }

        #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct $patch_name {
            $(pub $user_field: Option<$user_type>,)*
        }
    };
}
