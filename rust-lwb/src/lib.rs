// Mostly used in bootstrapper
pub use lwb_parser::bincode;

/// Code describing languages created with rust-lwb
pub use lwb_parser::language;

/// Helpers for code generation
pub mod transform;

/// Contains code related to syntax definitions
/// and parsing target languages based on these
/// definitions. Also contains sort/constructor
/// related code and code generation for these.
/// Contains the PEG parser.
pub use lwb_parser::parser;

/// Code related to generating rust source files
/// from language definitions. Usually used from
/// build.rs files.
pub use lwb_parser::codegen;

/// Contains code related to source code of languages
/// such as spans, and the [`SourceFile`] struct.
pub use lwb_parser::sources;

/// Collection of imports used in automatically generated
/// files (to avoid listing many imports in them). Should
/// not generally be used directly by users of rust-lwb
pub use lwb_parser::codegen_prelude;

/// Utilities to typecheck an abstract syntax tree.
pub mod typechecker;

/// Code related to configuring rust-lwb
pub use lwb_parser::config;

pub use lwb_parser::error;
