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

macro_rules! _annotation {

    (
        annotation_tokens = [$column_ident:ident = $column_value:expr, $($rest:tt)*],
        current_annotation = [ $($current_annotation:tt)* ],
        $($args:tt)*
    ) => {
        _annotation! {
            annotation_tokens = [$($rest)*],
            current_annotation = [
                {key = $column_ident , value = $column_value}, $($current_annotation)*
            ],
            $($args)*
        }
    };

    (
        annotation_tokens = [],
        current_annotation = [ $($current_annotation:tt)* ],
        tokens = [$($tokens:tt)*],
        $($args:tt)*
    ) => {
        _columns! {
            tokens = [$($tokens)*],
            annotations = [$($current_annotation)*],
            $($args)*
        };
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `anotation!` syntax.");
    }
}

macro_rules! _columns {
    (
        tokens = [#[$($annotation:tt)*] $($rest:tt)*],
        $($args:tt)*
    ) => {

        _annotation! {
           annotation_tokens = [$($annotation)*],
           current_annotation = [],
           tokens = [$($rest)*],
           $($args)*
        }
    };

    // we've annotations
    (
        tokens = [$token:ident : $ty:ty, $($rest:tt)*],
        annotations = [$($annotations:tt)*],
        $($args:tt)*
    ) => {
         _columns! {
            current_column = {
                type_name = $token,
                ty = $ty,
                annotations = [$($annotations)*],
            },
            tokens = [$($rest)*],
            $($args)*
        }
    };

    // we've no annotations
    (
        tokens = [$token:ident : $ty:ty, $($rest:tt)*],
        $($args:tt)*
    ) => {
         _columns! {
            current_column = {
                type_name = $token,
                ty = $ty,
                annotations = [ { key = phantom , value = "phantom" }, ],
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

#[derive(Debug)]
pub struct Update<'a> {
    pub field : &'static str,
    pub column : &'static str,
    pub new_value : &'a u32,
    pub old_value : &'a Option<u32>,
}

pub trait CalcUpdats {
    fn updates(&self) -> Vec<Update>;
}

macro_rules! _table_impl {
    (
        table = {
            struct_name = $struct_name:ident,
            table_name = $table_name:expr,
        },
        columns = [$({
            type_name = $type_name:ident,
            ty = $ty:ty,
            annotations = [$({
                key = $annotation_key:ident,
                value = $annotation_value:expr,
            },)*],
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

            pub fn get_pks() -> ! {
                $(
                    let x = vec![$(
                        (stringify!($annotation_key), $annotation_value)
                    )*];
                )*
            }

            async fn build_update_query(&self) -> Option<String> {

                let updates = self.updates();

                if updates.len() == 0 {
                    return None;
                }

                let mut sql = String::new();
                sql.push_str("UPDATE ");
                sql.push_str(stringify!($table_name));
                sql.push_str(" ");

                sql.push_str(
                    &updates
                        .iter()
                        .map(|u| format!("{} = ?", u.column))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                let mut q = sqlx::query(&sql);

                let pool = ::sqlx::MySqlPool::connect("mysql://user:pass@host/database").await.expect("...");


                for update in updates {
                    q = q.bind(update.new_value.clone());
                }

/*
                q
                .execute(&pool)
                .await.expect("...");
*/
                Some("sql".to_string())
            }
        }

        impl CalcUpdats for $struct_name {

            fn updates(&self) -> Vec<Update> {
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
            #[column = "foo column", xxx = "barrrr",]
            foo1 : u32,
            foo2 : u32,
        }
    );
    trace_macros!(false);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_diff() {

        table!(
            #[table = "bar_table"]
            struct Foo {
                #[column = "foo column",]
                foo1 : u32,
            }
        );

        let x = Foo {
            _shadow: table_shadow::Foo {
                foo1: None,
            },
            foo1: 123,
        };

        assert_eq!(1, CalcUpdats::updates(&x).len());

        let x = Foo {
            _shadow: table_shadow::Foo {
                foo1: Some(123),
            },
            foo1: 123,
        };

        assert_eq!(0, CalcUpdats::updates(&x).len());
    }

}