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
                type_name = $token,
                ty = $ty,
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
                type_name = $token,
                ty = $ty,
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

pub struct Update<'a> {
    pub field : &'static str,
    pub column : &'static str,
    pub new_value : &'a u64,
    pub old_value : &'a Option<u64>,
}

macro_rules! _table_impl {
    (
        table = {
            struct_name = $struct_name:ident,
            table_name = $table_name:expr,
        },
        columns = [$({
            column_name = $column_name:expr,
            type_name = $type_name:ident,
            ty = $ty:ty,
        },)+],
    ) => {

        mod table_shadow {
            pub struct $struct_name {
                $(
                    pub (crate) $type_name : Option<$ty>,
                )+
            }
        }

        pub struct $struct_name {
            _shadow : table_shadow::$struct_name,

            $(
                $type_name : $ty,
            )+
        }

        impl $struct_name {
            pub fn updates(&self) -> Vec<Update> {
                let mut buf = vec![];
                $(

                    match (&self.$type_name, &self._shadow.$type_name) {
                        (o, Some(s)) if o == s => {
                            // noop, same
                        },
                        _ => {
                            buf.push(Update {
                                field : stringify!($type_name),
                                column : stringify!($column_name),
                                new_value : &self.$type_name,
                                old_value : &self._shadow.$type_name,
                            });
                        }

                    };
                )+

                buf
            }
        }


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
            foo2 : u64,
        }
    );
    trace_macros!(false);

    /*
    let y = table_shadow::Foo {

    }

    let x = Foo {
    };
     */

}
