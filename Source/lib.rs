mod chunk;
mod joiner;
mod magic_string;
#[cfg(feature = "source_map")]
mod source_map;
mod span;
mod type_aliases;

type CowStr<'a> = Cow<'a, str>;

use std::borrow::Cow;

#[cfg(feature = "source_map")]
pub use crate::magic_string::source_map::SourceMapOptions;
pub use crate::{
	joiner::{Joiner, JoinerOptions},
	magic_string::{
		indent::IndentOptions,
		update::UpdateOptions,
		MagicString,
		MagicStringOptions,
	},
};
