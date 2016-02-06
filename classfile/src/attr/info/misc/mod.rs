pub mod annotations;
pub mod signature;

pub use self::annotations::RuntimeVisibleAnnotationsAttrInfo;
pub use self::signature::SignatureAttrInfo;

empty_attr_info!(SyntheticAttrInfo);
empty_attr_info!(DeprecatedAttrInfo);
