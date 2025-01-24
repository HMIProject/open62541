use std::hash;

crate::data_type!(FilterOperator, UInt32);

crate::enum_variants!(
    FilterOperator,
    UA_FilterOperator,
    [
        EQUALS,
        ISNULL,
        GREATERTHAN,
        LESSTHAN,
        GREATERTHANOREQUAL,
        LESSTHANOREQUAL,
        LIKE,
        NOT,
        BETWEEN,
        INLIST,
        AND,
        OR,
        CAST,
        INVIEW,
        OFTYPE,
        RELATEDTO,
        BITWISEAND,
        BITWISEOR,
    ]
);

impl hash::Hash for FilterOperator {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
