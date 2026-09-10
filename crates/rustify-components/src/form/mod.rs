//! Forms: the bookkeeping is the SDK's (`rustify_ui::FormState`), the values
//! are the application's, and what is here is the wiring between them and the
//! accessibility tree.

pub mod field;
pub mod provider;
pub mod submit;

pub use field::{Field, FieldBinding};
pub use provider::{provide_form, use_form, Form};
pub use submit::{FormStatus, SubmitButton};
