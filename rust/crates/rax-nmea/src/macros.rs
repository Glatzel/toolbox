macro_rules! readonly_struct {
    ($name:ident, $($struct_doc:expr)+, $({$field:ident: $type:ty $(, $field_doc:expr)?}),*) => {
        $(#[doc=$struct_doc])+
        #[derive(serde::Serialize, serde::Deserialize, Clone)]
        pub struct $name {
            $( $field: $type ),*
        }

        impl $name {
            // Getter methods for each field
            $(
                $(#[doc=$field_doc])?
                pub fn $field(&self) -> &$type {
                    &self.$field
                }
            )*
        }
    }
}

pub(crate) use readonly_struct;
