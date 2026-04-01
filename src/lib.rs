pub mod action;
pub mod annotation;
pub mod catalog;
pub mod color;
pub mod cross_reference_table;
pub mod encoding;
pub mod error;
pub mod extended_graphics_state;
pub mod generation;
pub mod gradient;
pub mod graphics_state;
pub mod metadata;
pub mod objects;
pub mod optional_content;
pub mod outline;
pub mod page;
pub mod pattern;
pub mod pdf_file;
pub mod resource;
pub mod resources;
pub mod text;
pub mod util;
pub mod writer;
pub mod fonts;
pub mod pdf_version;
pub mod page_size;
pub mod body;
pub mod header;
pub mod trailer;
pub mod file_identifier;

// Re-export main types for user API convenience
pub use action::{
    Action, FitDestination, GoToAction, JavaScriptAction, LaunchAction, NamedAction,
    NamedActionType, UriAction,
};
pub use annotation::{
    Annotation, AnnotationFlags, BorderStyle, LinkAction, LinkAnnotation, TextAnnotation, TextIcon,
};
pub use error::{PdfError, PdfResult};
pub use extended_graphics_state::{BlendMode, ExtGState, RenderingIntent};
pub use gradient::ColorStop;
pub use graphics_state::GraphicsStateManager;
pub use metadata::{DocumentInfo, TrappedState, XmpMetadata};
pub use objects::array::PdfArrayObject;
pub use objects::boolean::PdfBooleanObject;
pub use objects::dictionary::PdfDictionaryObject;
pub use objects::indirect::PdfIndirectObject;
pub use objects::metadata::PdfMetadata;
pub use objects::name::PdfNameObject;
pub use objects::null::PdfNullObject;
pub use objects::number_type::NumberType;
pub use objects::pdf_object::PdfObject;
pub use objects::stream::PdfStreamObject as Stream;
pub use objects::stream::{CompressionMethod, PdfStreamObject};
pub use objects::string::PdfStringObject;
pub use optional_content::{
    LayerOrder, OptionalContentConfig, OptionalContentGroup, VisibilityInitialState,
};
pub use outline::{DocumentOutline, OutlineItem, OutlineItemFlags};
pub use page::PageSize;
pub use pattern::{AxialShading, PaintType, PatternType, ShadingType, TilingPattern, TilingType};
pub use pdf_file::{PdfFile};
pub use resource::{Resource, ResourceCategory, ResourceManager};
pub use resources::ResourceMap;
pub use text::{StandardFont, WrapMode, wrap_text};
pub use util::Rect;
