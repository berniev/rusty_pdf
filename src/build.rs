use std::rc::Rc;

use crate::PdfObject;

/// Trait for types that can build a PDF object representation.
///
/// This trait provides a unified interface for building PDF objects from
/// various types (primitives, domain types, etc.).
///
/// # Examples
///
/// ```
/// use pydyf::Build;
///
/// let name_obj = "Page".build();
/// let num_obj = 42.build();
/// let bool_obj = true.build();
/// ```
pub trait Build {
    /// Build a PDF object representation of this value.
    fn build(self) -> Rc<dyn PdfObject>;
}
