use open62541_sys::UA_ExtensionObjectEncoding;

use crate::{ua, DataType};

crate::data_type!(ExtensionObject);

impl ExtensionObject {
    /// Gets encoded byte string content.
    pub fn encoded_content_bytestring(&self) -> Option<(&ua::NodeId, &ua::ByteString)> {
        match self.0.encoding {
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_BYTESTRING => {}
            _ => return None,
        }

        let encoded_content = unsafe { self.0.content.encoded.as_ref() };

        Some((
            ua::NodeId::raw_ref(&encoded_content.typeId),
            ua::ByteString::raw_ref(&encoded_content.body),
        ))
    }

    /// Gets encoded XML content.
    pub fn encoded_content_xml(&self) -> Option<(&ua::NodeId, &ua::String)> {
        match self.0.encoding {
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_XML => {}
            _ => return None,
        }

        let encoded_content = unsafe { self.0.content.encoded.as_ref() };

        Some((
            ua::NodeId::raw_ref(&encoded_content.typeId),
            ua::String::raw_ref(&encoded_content.body),
        ))
    }

    /// Gets decoded content.
    pub fn decoded_content<T: DataType>(&self) -> Option<&T> {
        match self.0.encoding {
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED
            | UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED_NODELETE => {}
            _ => return None,
        }

        let decoded_content = unsafe { self.0.content.decoded.as_ref() };

        (decoded_content.type_ == T::data_type()).then(|| {
            T::raw_ref(
                unsafe { decoded_content.data.cast::<T::Inner>().as_ref() }
                    .expect("data should be set"),
            )
        })
    }
}
