use crate::internal_prelude::*;
use crate::QString;

cpp! {{
    #include <QtCore/QLoggingCategory.h>
}}

cpp_class!(
    /// Wrapper around [`QLoggingCategory`][class] class.
    ///
    /// [class]: https://doc.qt.io/qt-5/qbytearray.html
    pub unsafe struct QLoggingCategory as "QLoggingCategory"
);

impl QLoggingCategory {
    pub fn set_filter_rules(rules: QString) {
        cpp!(unsafe [rules as "QString"] {
            QLoggingCategory::setFilterRules(rules)
        })
    }
}
