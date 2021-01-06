#![feature(trace_macros)]

macro_rules! table {
        ($($tokens:tt)*) => {
            _column! {
                tokens = [$($tokens)*],
                table_name = []
            }
    }
}

macro_rules! _column {
    (
        tokens = [#[column = $table_name:expr] $($rest:tt)*],
        $($args:tt)*
    ) => {
        _column! {
            tokens = [$($rest)*],
            column_name = $column_name,
            $($args)*
        }
    };

    (
        tokens = [$token:tt $($rest:tt)*],
        table_name = $column_name:expr,
        $($args:tt)*
    ) => {
         stringify!("consume token", $token, $column_name);
         _column! {
            tokens = [$($rest)*],
            column_name = [],
            $($args)*
        }
    };
}

fn main() {
    trace_macros!(true);
    table!(
        #[column = "foo column"]
        foo
         #[column = "foo column"]
        bar
    );
    trace_macros!(false);
}
