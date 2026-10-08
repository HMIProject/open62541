use open62541_sys::UA_ExtensionObjectEncoding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtensionObjectEncoding(pub(crate) u32);

macro_rules! impl_variants {
    ($t:ty, $($rest:tt)+) => {
        impl $t {
            impl_variants!($($rest)+);
        }
    };

    ($variant:ident = $discriminant:expr $(, $($rest:tt)+)?) => {
        paste::paste! {
            pub const $variant: Self = Self($discriminant);
            pub const [<$variant _U32>]: u32 = $discriminant;
            #[must_use]
            pub const fn [<is_ $variant:lower>](&self) -> bool {
                self.0 == Self::[<$variant _U32>]
            }
        }

        $(impl_variants!($($rest)+);)?
    };
}

impl_variants!(
    ExtensionObjectEncoding,
    ENCODED_NOBODY = UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_NOBODY.0,
    ENCODED_BYTESTRING = UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_BYTESTRING.0,
    ENCODED_XML = UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_XML.0,
    DECODED = UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED.0,
    DECODED_NODELETE = UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED_NODELETE.0
);

const _: () = {
    if std::mem::size_of::<UA_ExtensionObjectEncoding>()
        != std::mem::size_of::<ExtensionObjectEncoding>()
    {
        panic!("native `UA_ExtensionObjectEncoding` repr doesn't match Rust repr")
    }
};
