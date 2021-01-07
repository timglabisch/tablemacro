#![feature(trace_macros)]

macro_rules! table {
        ($($tokens:tt)*) => {
            _table! {
                tokens = [$($tokens)*],
            }
        }
}

macro_rules! _table {

    (
        tokens = [#[table = $table_name:expr] $($rest:tt)*],
        $($args:tt)*
    ) => {
        _table! {
            tokens = [$($rest)*],
            table_name = $table_name,
            $($args)*
        }
    };

    (
        tokens = [struct $struct_name:ident { $($rest:tt)* }],
        table_name = $table_name:expr,
        $($args:tt)*
    ) => {
            _columns! {
                tokens = [$($rest)*],
                table = {
                    struct_name = $struct_name,
                    table_name = $table_name,
                },
                columns = [],
                $($args)*
            }
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `table!` syntax.");
    }
}

macro_rules! _columns {
    (
        tokens = [#[column = $column_name:expr] $($rest:tt)*],
        $($args:tt)*
    ) => {
        _columns! {
            current_column = {
                column_name = $column_name,
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // we've the column name
    (
        current_column = {
            column_name = $column_name:expr,
        },
        tokens = [$token:ident : $ty:ty, $($rest:tt)*],
        $($args:tt)*
    ) => {
         _columns! {
            current_column = {
                column_name = $column_name,
                type_name: $token,
                ty: $ty
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // we've no column name, but it's fine, it's otional.
    (
        tokens = [$token:ident : $ty:ty, $($rest:tt)*],
        $($args:tt)*
    ) => {
         _columns! {
            current_column = {
                column_name = [],
                type_name: $token,
                ty: $ty
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // Done parsing this column
    (
        current_column = {
            $($current_column:tt)*
        },
        tokens = $tokens:tt,
        table = $table:tt,
        columns = [$($columns:tt,)*],
        $($args:tt)*
    ) => {
         _columns! {
            tokens = $tokens,
            table = $table,
            columns = [$($columns,)* { $($current_column)* },],
            $($args)*
        }
    };

    (
        tokens = [],
        $($args:tt)*
    ) => {
        _table_impl!($($args)*);
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `column!` syntax.");
    };
}

macro_rules! _table_impl {
    (
        table = $table:tt,
        columns = $columns:tt,
    ) => {
        stringify!(FINISH);
    };

     // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `_table_impl!` syntax.");
    };
}


fn main() {

    trace_macros!(true);
    table!(
        #[table = "bar_table"]
        struct Foo {
            #[column = "foo column"]
            foo1 : u64,
            foo2 : u32,
        }
    );
    trace_macros!(false);


}
