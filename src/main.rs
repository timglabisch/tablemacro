#![feature(trace_macros)]

mod static_cond;

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
        annotation_tokens = [pk, $($rest:tt)*],
        annotations = {
            primary_key = $old_primary_key:ident,
            column = $column_value:expr,
        },
        $($args:tt)*
    ) => {
        _annotation! {
            annotation_tokens = [$($rest)*],
            annotations = {
                primary_key = true,
                column = $column_value,
            },
            $($args)*
        }
    };

    (
        annotation_tokens = [column = $column_value:expr, $($rest:tt)*],
        annotations = {
            primary_key = $primary_key:ident,
            column = $column_value_old:expr,
        },
        $($args:tt)*
    ) => {
        _annotation! {
            annotation_tokens = [$($rest)*],
            annotations = {
                primary_key = $primary_key,
                column = $column_value,
            },
            $($args)*
        }
    };

    (
        annotation_tokens = [],
        annotations = $annotations:tt,
        tokens = [$($tokens:tt)*],
        $($args:tt)*
    ) => {
        _columns! {
            tokens = [$($tokens)*],
            annotations = $annotations,
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
           annotations = {
            primary_key = false,
            column = "",
           },
           tokens = [$($rest)*],
           $($args)*
        }
    };

    // we've annotations
    (
        tokens = [$token:ident : $ty:ty, $($rest:tt)*],
        annotations = {
            primary_key = $primary_key:ident,
            column = $column_value:expr,
        },
        $($args:tt)*
    ) => {
         _columns! {
            current_column = {
                type_name = $token,
                ty = $ty,
                primary_key = $primary_key,
                column = $column_value,
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
                primary_key = false,
                column = "",
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
        _table_impl!(
            $($args)*
        );
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `column!` syntax.");
    };
}

#[derive(Debug)]
pub struct Update<'a> {
    pub field: &'static str,
    pub column: &'static str,
    pub new_value: &'a u32,
    pub old_value: Option<&'a u32>,
}

pub enum Change<'a> {
    Update(Vec<Update<'a>>),
    Insert,
}

impl<'a> Change<'a> {
    pub fn unwrap_update(self) -> Vec<Update<'a>> {
        match self {
            Self::Update(u) => u,
            _ => panic!("expected update."),
        }
    }
}

pub trait CalcUpdats {
    fn changes(&self) -> Change;
}




macro_rules! _separated {

    ([$($separator:tt)*] $($tokens:tt)*) => {
        _separated! {
            separator = [$($separator)*],
            tokens = [$($tokens)*],
            result = [],
        }
    };

    (
        separator = [$($separator:tt)*],
        tokens = [$token:tt,],
        result = [$($result:tt)*],
    ) => {
        _separated! {
            separator = [$($separator)*],
            tokens = [],
            result = [$($result)* $token],
        }
    };

    (
        separator = [$($separator:tt)*],
        tokens = [$token:tt, $($rest:tt)*],
        result = [$($result:tt)*],
    ) => {
        _separated! {
            separator = [$($separator)*],
            tokens = [$($rest)*],
            result = [$($result)* $token $($separator)*],
        }
    };

    (
        separator = [$($separator:tt)*],
        tokens = [],
        result = [$($result:tt)*],
    ) => {
        concat!($($result)*)
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `_seperated!` syntax.");
    };
}


macro_rules! _instead_separated {

    ([$($instead:tt)*] [$($separator:tt)*] $($tokens:tt)*) => {
        _instead_separated! {
            instead = [$($instead)*],
            separator = [$($separator)*],
            tokens = [$($tokens)*],
            result = [],
        }
    };

    // consume ,
    (
        instead = [$($instead:tt)*],
        separator = [$($separator:tt)*],
        tokens = [,$($rest:tt)*],
        result = [$($result:tt)*],
    ) => {
        _instead_separated! {
            instead = [$($instead)*],
            separator = [$($separator)*],
            tokens = [$($rest)*],
            result = [$($result)*],
        }
    };

    (
        instead = [$($instead:tt)*],
        separator = [$($separator:tt)*],
        tokens = [$token:tt ,],
        result = [$($result:tt)*],
    ) => {
        _instead_separated! {
            instead = [$($instead)*],
            separator = [$($separator)*],
            tokens = [],
            result = [$($result)* $($instead)*],
        }
    };

    (
        instead = [$($instead:tt)*],
        separator = [$($separator:tt)*],
        tokens = [$token:tt $($rest:tt)+],
        result = [$($result:tt)*],
    ) => {
        _instead_separated! {
            instead = [$($instead)*],
            separator = [$($separator)*],
            tokens = [$($rest)*],
            result = [$($result)* $($instead)* $($separator)*],
        }
    };

    (
        instead = [$($instead:tt)*],
        separator = [$($separator:tt)*],
        tokens = [],
        result = [$($result:tt)*],
    ) => {
        concat!($($result)*)
    };

    // Invalid syntax
    ($($tokens:tt)*) => {
        compile_error!("Invalid `_instead_separated!` syntax.");
    };
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
            primary_key = $primary_key:ident,
            column = $column:expr,
        },)*],
    ) => {

        mod table_shadow {
            pub struct $struct_name {
                $(
                    pub (crate) $type_name : $ty,
                )+
            }
        }

        pub struct $struct_name {
            _shadow : Option<table_shadow::$struct_name>,

            $(
                $type_name : $ty,
            )+
        }

        impl $struct_name {

            pub fn get_pks() -> Vec<&'static str> {

                // todo, find a way without heap allocation
                let mut pk = vec![];
                $($crate::static_cond! {
                    if $primary_key == true {
                        pk.push($column);
                    } else {
                    }
                })*;

                pk
            }

            pub fn get_columns() -> &'static[&'static str] {
                &[$(
                    $column,
                )*]
            }

            async fn execute_update_query<'a>(&'a self, pool : ::sqlx::Pool<::sqlx::mysql::MySql>, sql : &str, updates : &Vec<Update<'a>>) -> Result<::sqlx::mysql::MySqlDone, ::sqlx::Error> {

                let mut q = ::sqlx::query::<::sqlx::mysql::MySql>(sql);

                for update in updates {
                    q = q.bind(update.new_value.clone());
                }

                $($crate::static_cond! {
                    if $primary_key == true {
                        q = q.bind(self.$type_name.clone());
                    }
                })*;

                let mut conn : ::sqlx::pool::PoolConnection<::sqlx::mysql::MySql> = pool.acquire().await.expect("nooo");

                q
                    .execute(&mut conn)
                    .await
            }

            async fn save(&self, pool : ::sqlx::Pool<::sqlx::mysql::MySql>)
                -> Option<Result<::sqlx::mysql::MySqlDone, ::sqlx::Error>>
            {
                match self.changes() {
                    Change::Update(u) => self.build_update_query(pool, u).await,
                    Insert => self.build_insert_query(pool).await
                }
            }


            async fn build_insert_query<'a>(&'a self, pool : ::sqlx::Pool<::sqlx::mysql::MySql>)
                -> Option<Result<::sqlx::mysql::MySqlDone, ::sqlx::Error>>
            where
             $(
             $ty: for<'x> ::sqlx::Encode<'x, ::sqlx::mysql::MySql>,
             $ty: ::sqlx::Type<::sqlx::mysql::MySql>,
             )*
             {
                let sql = concat!("INSERT INTO ", $table_name, " (", _separated! { [,",",] $($column,)* }, ") VALUES ( ", _instead_separated! { ["?"] [,", ",] $($column,)* } , " )");
                panic!("...");
             }

            async fn build_update_query<'a>(&'a self, pool : ::sqlx::Pool<::sqlx::mysql::MySql>, updates : Vec<Update<'a>>)
                -> Option<Result<::sqlx::mysql::MySqlDone, ::sqlx::Error>>
            where
             $(
             $ty: for<'x> ::sqlx::Encode<'x, ::sqlx::mysql::MySql>,
             $ty: ::sqlx::Type<::sqlx::mysql::MySql>,
             )*
             {

                let updates = self.changes().unwrap_update();

                if updates.len() == 0 {
                    return None;
                }

                let mut sql = String::new();
                sql.push_str("UPDATE ");
                sql.push_str(stringify!($table_name));
                sql.push_str(" SET ");

                sql.push_str(
                    &(&updates
                        .iter()
                        .map(|u| format!("{} = ?", u.column))
                        .collect::<Vec<_>>()
                        .join(", ")
                        )
                );

                sql.push_str("WHERE ");

                $($crate::static_cond! {
                    if $primary_key == true {
                        sql.push_str("`");
                        sql.push_str($column);
                        sql.push_str("` = ? AND ");
                    }
                })*;

                sql.push_str(" 1 = 1 ");

                Some(
                    self.execute_update_query(pool, &sql, &updates).await
                )
            }
        }

        impl CalcUpdats for $struct_name {

            fn changes(&self) -> Change {

                let shadow = match self._shadow {
                    Some(ref s) => s,
                    None => return Change::Insert
                };


                let mut buf = vec![];
                $(
                    if &shadow.$type_name != &self.$type_name {
                        buf.push(Update {
                            field : stringify!($type_name),
                            column : stringify!($column_name),
                            new_value : &self.$type_name,
                            old_value : match &self._shadow {
                                None => None,
                                Some(s) => Some(&s.$type_name)
                            },
                        });
                    }
                )+

                Change::Update(buf)
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
            #[pk, column = "foo_column",]
            foo1 : u32,
            #[pk, column = "bar_column",]
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
            _shadow: Some(table_shadow::Foo {
                foo1: 124,
            }),
            foo1: 123,
        };

        assert_eq!(1, CalcUpdats::changes(&x).unwrap_update().len());

        let x = Foo {
            _shadow: Some(table_shadow::Foo {
                foo1: 123,
            }),
            foo1: 123,
        };

        assert_eq!(0, CalcUpdats::changes(&x).unwrap_update().len());
    }

    #[test]
    fn integrationtest_update() {
        ::tokio::runtime::Builder::new()
            .basic_scheduler()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let database_url = "mysql://root:dummy@localhost:3306";
                let db_pool = ::sqlx::mysql::MySqlPool::connect(&database_url).await.expect("could not connect");
                ::sqlx::query("DROP DATABASE IF EXISTS test").execute(&db_pool).await.expect("drop db");
                ::sqlx::query("CREATE DATABASE test;").execute(&db_pool).await.expect("create db");
                let database_url = "mysql://root:dummy@localhost:3306/test";
                let db_pool = ::sqlx::mysql::MySqlPool::connect(&database_url).await.expect("could not connect");
                ::sqlx::query(r"
                CREATE TABLE `xxx`
                (
                    `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
                    `some_field`          int(11)        NOT NULL,
                    `some_other_field`    int(11)        NOT NULL,
                    PRIMARY KEY (`id`)
                ) ENGINE = InnoDB
                  DEFAULT CHARSET = utf8mb4
                  COLLATE = utf8mb4_unicode_ci;
                ").execute(&db_pool).await.expect("create db");

                table!(
                    #[table = "xxxx"]
                    struct Xxx {
                        #[pk, column = "id",]
                        id : u32,
                        #[column = "some_field",]
                        some_field : u32,
                        #[column = "some_other_field",]
                        some_other_field : u32,
                    }
                );

                let item = Xxx {
                    _shadow: None,
                    id: 1,
                    some_field: 2,
                    some_other_field: 3,
                };

                item.save(db_pool).await.expect("insert into db");
            })
    }
}