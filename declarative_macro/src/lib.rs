//! `$a: expr`: 表示符号a是一个表达式。其它可用的类型有：
//! - item: an item, like a function, struct, module, etc.
//! - block: a block of code, like { ... }
//! - stmt: a statement, like a local variable declaration: let x = 3;
//! - pat: a pattern, like Some(t)
//! - expr: an expression, like 2 + 2. 和 stmt 的区别是，expr 不需要分号结尾。表达式会返回值；
//! - ty: a type, like i32
//! - ident: a variable name, function name, or any other identifier
//! - path: a path like std::mem::replace, self::foo, or super::bar
//! - meta: a meta item, like cfg(target_os = "linux"), or deprecated. the things that go inside #[...] and #![...] attributes
//! - tt: a single token tree, like (a, b) or { a + b }. tt 是一个特殊的类型，它可以匹配任何类型的 token tree。
//! - vis: a visibility qualifier, like pub or pub(crate)
//! 声明宏的问题：
//! - Lack of support for macros autocompletion and expansion
//! - Debugging declarative macros is difficult
//! - Limited modification capabilities
//! - Larger binaries
//! - Longer compile time (this applies to both declarative and procedural macros)
//!

#[macro_export]
/// 初始版本，只支持两个输入。
macro_rules! add_v1 {
    ($a:expr, $b:expr) => {{
        $a + $b
    }};
}

#[macro_export]
macro_rules! add_as_v1 {
    ($a:expr, $b:expr, $t: ty) => {
        $a as $t + $b as $t
    };
}

/// 支持任意输入版本。
/// 这个版本的问题：生成的代码中，有一个 0 开头，这个 0 是没有意义的。
#[macro_export]
macro_rules! add_v2 {

    (
        // 重复部分
        $($a:expr)
        // 分隔符
        ,
        // 0 或多个重复
        *
    ) => {
        0 // 为保证语法正确，这里必须有一个初始值
        $(+$a)* // +a 部分进行重复
    };
}

/// 支持任意输入版本。
/// 通过 match 来解决 add_v2 的问题，即不再有无意义的 0 开头，并兼容没有参数的情况。
#[macro_export]
macro_rules! add_v3 {
    // 第零种情况：没有输入
    () => {
        0
    };
    // 第一种情况：只有一个输入
    ($a:expr) => {
        $a
    };
    // 第二种情况：2个输入
    ($a:expr, $b:expr) => {{
        $a + $b
    }};
    // 第三种情况：多于2个输入
    ($a:expr, $($b:tt)*) => {{
        $a+add_v3!($($b)*)
    }};
}

/// 将struct的可见性设置为pub。
/// 此版本不支持元数据。
#[macro_export]
macro_rules! make_public {
    ($vis:vis struct $struct_name:ident{
        // 重复部分，即字段定义。
        $(
            // 字段可见性
            $field_vis:vis
            // 字段名
            $field_name:ident
            // 字段类型
            : $field_type:ty
            // 分隔符，字段定义之间用逗号分隔。
            ,
        )
        // 0 或多个重复
        *
    }) => {
        pub struct $struct_name {
            $(
                pub $field_name: $field_type,
            )*
        }
    }
}

/// 支持 meta 的版本。
#[macro_export]
macro_rules! make_public_v2 {
    (
        // struct 的 meta
        $(#[$meta:meta])*
        $vis:vis
        struct
        $struct_name:ident
        {

            // 重复部分，即字段定义。
            $(
                // 字段的 meta
                $(#[$field_meta:meta])*
                // 字段可见性
                $field_vis:vis
                // 字段名
                $field_name:ident
                // 字段类型
                : $field_type:ty
                // 分隔符，字段定义之间用逗号分隔。
                ,
            )
            // 0 或多个重复
            *
        }
    ) => {
        // meta 保持不变
        $(#[$meta])*
        pub struct $struct_name {
            $(
                $(#[$field_meta])*
                pub $field_name: $field_type,
            )*
        }
    };
}

/// A macro that returns early from a function if the expression is an error.
#[macro_export]
macro_rules! ok_or_return {
    // e 匹配 some_func(...) 的模式，e 的值为 some_func。
    ($e:ident($($arg:tt)*)) => {
        match $e($($arg)*) {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
}

#[macro_export]
/// 使用内部宏的版本。
macro_rules! ok_or_return2 {
    // 内部宏，不对外暴露。避免命名冲突。
    (@any_name $a:ident,$($b:tt)*) => {{
        match $a($($b)*) {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    }};

    // 公开宏，对外暴露。
    ($e:ident($($arg:tt)*)) => {
        ok_or_return2!(@any_name $e, $($arg)*)
    };
}
