use fluent_bundle::{FluentArgs, FluentValue};

pub mod bundles {
    use fluent_bundle::{concurrent::FluentBundle, FluentResource};
    use std::sync::LazyLock;
    use std::sync::OnceLock;
    use unic_langid::langid;

    use super::functions::*;

    type Bundle = FluentBundle<FluentResource>;

    macro_rules! bundle {
        ($var:ident = $id:literal) => {
            pub static $var: LazyLock<Bundle> = LazyLock::new(|| {
                let mut bundle = Bundle::new_concurrent(vec![langid!($id)]);
                bundle.add_builtins().unwrap();
                bundle.add_function("STYLE", style).unwrap();
                bundle.add_function("IS_MINIMAL", is_minimal).unwrap();
                // .ftl files are checked at compile time in `build.rs`
                let resource = FluentResource::try_new(
                    include_str!(concat!("../localization/", $id, "/messages.ftl")).into(),
                )
                .unwrap();
                match bundle.add_resource(resource) {
                    Ok(()) => (),
                    Err(errs) => {
                        for err in errs {
                            eprintln!("ftl error: {err}");
                        }
                        panic!("Failed to add {} locale", $id);
                    }
                }
                bundle
            });
        };
    }

    bundle!(EN_US = "en-US");
    bundle!(TOK = "tok");

    pub fn locale() -> &'static [&'static LazyLock<Bundle>] {
        LOCALE.get_or_init(init_from_env)
    }

    pub fn init_from_env() -> &'static [&'static LazyLock<Bundle>] {
        let lang = std::env::var("LC_MESSAGES")
            .or_else(|_| std::env::var("LANG"))
            .unwrap_or_default();
        init_to(&lang)
    }

    pub fn init_to(s: &str) -> &'static [&'static LazyLock<Bundle>] {
        match s {
            "en-US" => const { &[&EN_US] },
            "tok" => const { &[&TOK, &EN_US] },
            _ => const { &[&EN_US] },
        }
    }

    pub static LOCALE: OnceLock<&[&LazyLock<Bundle>]> = OnceLock::new();
}

mod functions {
    use fluent_bundle::{FluentArgs, FluentValue};

    pub fn style<'a>(positional: &[FluentValue<'a>], _: &FluentArgs<'_>) -> FluentValue<'a> {
        let special = crate::special_render();
        let mut out = String::new();
        for arg in positional {
            let FluentValue::String(style) = arg else {
                continue;
            };
            let ansi = match style.as_ref() {
                "red" => special.red,
                "bright-red" => special.bright_red,
                "green" => special.green,
                "bright-green" => special.bright_green,
                "blue" => special.blue,
                "bright-blue" => special.bright_blue,
                "cyan" => special.cyan,
                "bright-cyan" => special.bright_cyan,
                "yellow" => special.yellow,
                "bright-yellow" => special.bright_yellow,
                "magenta" => special.magenta,
                "bright-magenta" => special.bright_magenta,
                "black" => special.black,
                "dark-grey" | "dark-gray" => special.dark_grey,
                "dark-grey-bg" | "dark-gray-bg" => special.dark_grey_bg,
                "light-grey" | "light-gray" => special.light_grey,
                "white" => special.white,
                "no_fg" => special.no_fg,
                "no_bg" => special.no_bg,
                "reset" => special.reset,

                "italic" => special.italic,
                "bold" => special.bold,
                "strike" => special.strike,
                "no-italic-bold" => special.no_italic_bold,
                "no-strike" => special.no_strike,
                _ => return FluentValue::Error,
            };
            out.push_str(ansi);
        }
        out.into()
    }

    pub fn is_minimal<'a>(_: &[FluentValue<'a>], _: &FluentArgs<'_>) -> FluentValue<'a> {
        if crate::special_render().fancy {
            "no".into()
        } else {
            "yes".into()
        }
    }
}

#[macro_export]
macro_rules! ftl_arg {
    ($args:ident, $var_name:ident) => {
        $args.set(stringify!($var_name), $var_name);
    };
    ($args:ident, $var_name:ident, $var_value:expr) => {
        $args.set(stringify!($var_name), $var_value);
    };
}

#[macro_export]
macro_rules! ftl_args {
    ($msg_id:expr) => {
        $crate::ftl_args!($msg_id,)
    };
    ($msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            $crate::localization::bundles::locale()
                .into_iter()
                .filter_map(|l| Some((l, l.get_message($msg_id)?.value()?)))
                .next()
                .map(|(bundle, pattern)| {
                    #[allow(unused_mut)]
                    let mut args = fluent_bundle::FluentArgs::new();
                    $(
                        $crate::ftl_arg!(args, $var_name$(, $var_val)*);
                    )*
                    (bundle, pattern, args)
                })
        }
    };
}

#[macro_export]
macro_rules! ftl_format {
    ($msg_id:expr) => {
        $crate::ftl_format!($msg_id,)
    };
    ($msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            let args = $crate::ftl_args!($msg_id, $($var_name $(= $var_val)*),*);
            if let Some((bundle, pattern, args)) = args {
                let mut errors = Vec::new();
                let out = bundle.format_pattern(pattern, Some(&args), &mut errors);
                if !errors.is_empty() {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    panic!("failed to format localized text");
                }
                out
            } else {
                std::borrow::Cow::from($msg_id)
            }

        }
    }
}

#[macro_export]
macro_rules! ftl_write {
    ($writer:expr, $msg_id:expr) => {
        $crate::ftl_write!($writer, $msg_id,)
    };
    ($writer:expr, $msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            use std::fmt::Write;
            let args = $crate::ftl_args!($msg_id, $($var_name $(= $var_val)*),*);
            if let Some((bundle, pattern, args)) = args {
                let mut errors = Vec::new();
                bundle.write_pattern($writer, pattern, Some(&args), &mut errors).expect("failed to write localized text");
                if !errors.is_empty() {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    panic!("failed to format localized text");
                }
            } else {
                write!($writer, "{}", $msg_id).expect("failed to write text message id");
            }
        }
    }
}

pub struct WriterCompat<W>(pub W);

impl<W: std::io::Write> std::fmt::Write for WriterCompat<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}

#[macro_export]
macro_rules! ftl_print {
    ($msg_id:expr) => {
        $crate::ftl_print!($msg_id,)
    };
    ($msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            let mut stdout = $crate::localization::WriterCompat(std::io::stdout());
            $crate::ftl_write!(&mut stdout, $msg_id, $($var_name $(= $var_val)*),*);
        }
    }
}

#[macro_export]
macro_rules! ftl_println {
    ($msg_id:expr) => {
        $crate::ftl_println!($msg_id,)
    };
    ($msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            use std::fmt::Write;
            let mut stdout = $crate::localization::WriterCompat(std::io::stdout());
            $crate::ftl_write!(&mut stdout, $msg_id, $($var_name $(= $var_val)*),*);
            writeln!(&mut stdout).expect("failed to write newline");
        }
    }
}

#[macro_export]
macro_rules! ftl_eprint {
    ($msg_id:expr) => {
        $crate::ftl_eprint!($msg_id,)
    };
    ($msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            let mut stderr = $crate::localization::WriterCompat(std::io::stderr());
            $crate::ftl_write!(&mut stderr, $msg_id, $($var_name $(= $var_val)*),*);
        }
    }
}

#[macro_export]
macro_rules! ftl_eprintln {
    ($msg_id:expr) => {
        $crate::ftl_eprintln!($msg_id,)
    };
    ($msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            use std::fmt::Write;
            let mut stderr = $crate::localization::WriterCompat(std::io::stderr());
            $crate::ftl_write!(&mut stderr, $msg_id, $($var_name $(= $var_val)*),*);
            writeln!(&mut stderr).expect("failed to write newline");
        }
    }
}

#[macro_export]
macro_rules! ftl_eyre {
    ($msg_id:expr) => {
        $crate::ftl_eyre!($msg_id,)
    };
    ($msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            eyre::eyre!("{}", $crate::ftl_format!($msg_id, $($var_name $(= $var_val)*),*))
        }
    }
}

#[macro_export]
macro_rules! ftl_bail {
    ($msg_id:expr) => {
        $crate::ftl_bail!($msg_id,)
    };
    ($msg_id:expr, $($var_name:ident $(= $var_val:expr)?),*) => {
        {
            eyre::bail!("{}", $crate::ftl_format!($msg_id, $($var_name $(= $var_val)*),*))
        }
    }
}
