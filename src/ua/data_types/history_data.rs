use crate::{DataType as _, ua};

crate::data_type!(HistoryData);

impl HistoryData {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.dataValuesSize == 0
    }
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.dataValuesSize
    }

    #[must_use]
    pub fn get(&self, i: usize) -> Option<ua::DataValue> {
        if i < self.0.dataValuesSize {
            Some(unsafe { ua::DataValue::clone_raw(&self.0.dataValues.add(i).read()) })
        } else {
            None
        }
    }
}
