use crate::create_spaces;
use tarantool_test::{bind_test_suite, TestSuite};

struct TodoAppTestSuite;

impl TestSuite for TodoAppTestSuite {
    fn before_all() {
        let _ = create_spaces();
    }
}

bind_test_suite!(TodoAppTestSuite);
