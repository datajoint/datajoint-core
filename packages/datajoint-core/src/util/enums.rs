pub(crate) mod enums {
    macro_rules! generate_primitive_datajoint_enum {
        ($(#[$meta:meta])* $vis:vis enum $name:ident {
            $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
        }) => {
            $(#[$meta])*
            $vis enum $name {
                $($(#[$vmeta])* $vname $(= $val)?,)*
            }

            impl std::convert::TryFrom<i32> for $name {
                type Error = ();
                fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
                    match v {
                        $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                        _ => Err(()),
                    }
                }
            }
        }
    }

    pub(crate) use generate_primitive_datajoint_enum;
}
