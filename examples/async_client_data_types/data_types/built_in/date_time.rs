// [Part 3: 8.11 DateTime](https://reference.opcfoundation.org/Core/Part3/v105/docs/8.11)
// [Part 5: 12.2.4 DateTime](https://reference.opcfoundation.org/Core/Part5/v105/docs/12.2.4)
// [Part 6: 5.1.4 DateTime](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.1.4)
// [Part 6: 5.2.2.5 DateTime](https://reference.opcfoundation.org/Core/Part6/v105/docs/5.2.2.5)
pub struct DateTime(pub i64);

impl DateTime {
    pub fn min_value() -> Self {
        Self(0)
    }

    pub fn max_value() -> Self {
        Self(i64::MAX)
    }
}
