pub mod encoding;
pub mod error;
pub mod gradient;
pub mod graphics_state;
pub mod objects;
pub mod pdf;
pub mod resources;
pub mod text;
pub mod page;
pub mod util;
pub mod writer;
pub mod color;
pub mod cross_ref;
pub mod catalog;

// Re-export main types for user API convenience
pub use objects::dictionary::DictionaryObject;
pub use objects::stream::StreamObject;
pub use objects::pdf_object::PdfObject;
pub use objects::number::{NumberObject, NumberType};
pub use objects::name::NameObject;
pub use objects::string::StringObject;
pub use error::{PdfError, PdfResult};
pub use gradient::{ColorStop};
pub use graphics_state::GraphicsStateManager;
pub use pdf::{FileIdentifierMode, PDF};
pub use resources::ResourceDictionary;
pub use text::{wrap_text, StandardFont, WrapMode};
pub use page::{PageObject};
