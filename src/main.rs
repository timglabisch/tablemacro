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
        _column! {
            tokens = [$($rest)*],
            struct_name = $struct_name,
            column_name = [],
            table_name = $table_name,
            $($args)*
        }
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `table!` syntax.");
    }
}

macro_rules! _column {
    (
        tokens = [#[column = $column_name:expr] $($rest:tt)*],
        struct_name = $struct_name:ident,
        $($args:tt)*
    ) => {
        _column! {
            tokens = [$($rest)*],
            column_name = $column_name,
            struct_name = $struct_name,
            $($args)*
        }
    };

    (
        tokens = [$token:ident : $type:ty, $($rest:tt)*],
        column_name = $column_name:expr,
        struct_name = $struct_name:ident,
        $($args:tt)*
    ) => {
         stringify!(token = $token, type = $type, column_name = $column_name, struct_name=$struct_name);
         _column! {
            tokens = [$($rest)*],
            column_name = [],
            struct_name = $struct_name,
            $($args)*
        }
    };


    (
        tokens = [],
        $($args:tt)*
    ) => {
         stringify!(FINISH);
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `column!` syntax.");
    };
}

macro_rules! _test {
    ($token:tt : $type:ty $(,$rest:tt)*) => { stringify!(OK); };
    ($($tokens:tt)*) => {
        compile_error!("Invalid!");
    }
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
