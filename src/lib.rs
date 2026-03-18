pub mod action;
pub mod annotation;
pub mod catalog;
pub mod color;
pub mod cross_ref;
pub mod encoding;
pub mod error;
pub mod extended_graphics_state;
pub mod gradient;
pub mod graphics_state;
pub mod metadata;
pub mod objects;
pub mod optional_content;
pub mod outline;
pub mod page;
pub mod pattern;
pub mod pdf;
pub mod resource;
pub mod resources;
pub mod text;
pub mod util;
pub mod writer;

// Re-export main types for user API convenience
pub use action::{
    Action, FitDestination, GoToAction, JavaScriptAction, LaunchAction, NamedAction, NamedActionType,
    UriAction,
};
pub use annotation::{
    Annotation, AnnotationFlags, BorderStyle, LinkAction, LinkAnnotation, TextAnnotation, TextIcon,
};
pub use error::{PdfError, PdfResult};
pub use extended_graphics_state::{BlendMode, ExtGState, RenderingIntent};
pub use gradient::ColorStop;
pub use graphics_state::GraphicsStateManager;
pub use metadata::{DocumentInfo, TrappedState, XmpMetadata};
pub use objects::array::ArrayObject;
pub use objects::boolean::BooleanObject;
pub use objects::dictionary::DictionaryObject;
pub use objects::indirect::IndirectObject;
pub use objects::metadata::PdfMetadata;
pub use objects::name::NameObject;
pub use objects::number::{NumberObject, NumberType};
pub use objects::pdf_object::PdfObject;
pub use objects::stream::StreamObject as Stream;
pub use objects::stream::{CompressionMethod, StreamObject};
pub use objects::string::StringObject;
pub use optional_content::{
    LayerOrder, OptionalContentConfig, OptionalContentGroup, VisibilityInitialState,
};
pub use outline::{DocumentOutline, OutlineItem, OutlineItemFlags};
pub use page::{PageObject, PageSize};
pub use pattern::{AxialShading, PaintType, PatternType, ShadingType, TilingPattern, TilingType};
pub use pdf::{FileIdentifierMode, PDF};
pub use resource::{Resource, ResourceCategory, ResourceManager};
pub use resources::ResourceDictionary;
pub use text::{StandardFont, WrapMode, wrap_text};
pub use util::Rect;
