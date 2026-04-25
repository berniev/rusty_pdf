pub mod action;
pub mod annotation;
pub mod body;
pub mod catalog;
pub mod color;
pub mod cross_ref_stream;
pub mod xref_ops;
pub mod drawing_commands;
pub mod encoding;
pub mod encryption;
pub mod error;
pub mod ext_g_state;
pub mod file_identifier;
pub mod file_specification;
pub mod fonts;
pub mod function;
pub mod generation;
pub mod graphics_ops;
pub mod header;
pub mod metadata;
pub mod object_ops;
pub mod objects;
pub mod optional_content;
pub mod outline;
pub mod page_ops;
pub mod page_size;
pub mod pattern;
pub mod pdf;
pub mod resource_category;
pub mod resources_ops;
pub mod shading;
pub mod soft_mask;
pub mod string_functions;
pub mod text;
pub mod trailer;
pub mod tree;
pub mod util;
pub mod version;
pub mod xmp;
pub mod encryption_ops;

// Re-export main types for user API convenience
pub use action::{
    Action, FitDestination, GoToAction, JavaScriptAction, LaunchAction, NamedAction,
    NamedActionType, UriAction,
};
pub use annotation::{
    Annotation, AnnotationFlags, BorderStyle, LinkAction, LinkAnnotation, TextAnnotation, TextIcon,
};
pub use error::{PdfError, PdfResult};
pub use ext_g_state::{BlendMode, ExtGState, RenderingIntent};
pub use graphics_ops::GraphicsOps;
pub use metadata::{Metadata, TrappedState};
pub use objects::pdf_array::PdfArrayObject;
pub use objects::pdf_boolean::PdfBooleanObject;
pub use objects::pdf_dictionary::PdfDictionaryObject;
pub use objects::pdf_name::PdfNameObject;
pub use objects::pdf_null::PdfNullObject;
pub use objects::number_type::NumberType;
pub use objects::pdf_object::PdfObject;
pub use objects::pdf_reference::PdfReferenceObject;
pub use objects::pdf_stream::PdfStreamObject as Stream;
pub use objects::pdf_stream::{CompressionMethod, PdfStreamObject};
pub use objects::pdf_string::PdfStringObject;
pub use optional_content::{
    LayerOrder, OptionalContentConfig, OptionalContentGroup, VisibilityInitialState,
};
pub use outline::{DocumentOutline, OutlineItem, OutlineItemFlags};
pub use page_ops::PageSize;
pub use pattern::{PaintType, PatternType, TilingPattern, TilingType};
pub use pdf::Pdf;
pub use resource_category::ResourceCategory;
pub use resources_ops::NamedResources;
pub use text::{StandardFont, WrapMode, wrap_text};
pub use util::Rectangle;
