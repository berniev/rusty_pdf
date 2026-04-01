//! Action framework for interactive PDF behaviors.
//!
//! Actions define behaviors that can be triggered by user interactions, such as
//! clicking links, opening documents, or interacting with form fields.
/// Actions specify responses to various events in PDF documents, such as
/// user interactions with annotations or form fields.

use crate::objects::pdf_object::Pdf;
use crate::util::Rect;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfResult};

//------------------------ Action -------------------------------//

pub trait Action {
    fn action_type(&self) -> &'static str; // URI, GoTo, JavaScript, etc.
    fn to_dict(&self) -> PdfResult<PdfDictionaryObject>;
}

pub struct UriAction {
    pub uri: String,
    pub is_map: bool,
}

impl UriAction {
    pub fn new(uri: String) -> Self {
        Self { uri, is_map: false }
    }

    pub fn with_is_map(mut self, is_map: bool) -> Self {
        self.is_map = is_map;
        self
    }
}

impl Action for UriAction {
    fn action_type(&self) -> &'static str {
        "URI"
    }

    fn to_dict(&self) -> PdfResult<PdfDictionaryObject> {
        let mut dict = PdfDictionaryObject::new();
        dict.add("S", Pdf::name(self.action_type()));
        dict.add("URI", Pdf::string(self.uri.as_str()));

        if self.is_map {
            dict.add("IsMap", Pdf::bool(true));
        }

        Ok(dict)
    }
}

pub struct GoToAction {
    pub destination: FitDestination,
}

impl GoToAction {
    pub fn new(destination: FitDestination) -> Self {
        Self { destination }
    }
}

impl Action for GoToAction {
    fn action_type(&self) -> &'static str {
        "GoTo"
    }

    fn to_dict(&self) -> PdfResult<PdfDictionaryObject> {
        let mut dict = PdfDictionaryObject::new();
        dict.add("S", Pdf::name(self.action_type()));
        dict.add("D", Pdf::array(self.destination.to_pdf_array()));
        Ok(dict)
    }
}

pub struct JavaScriptAction {
    pub script: String,
}

impl JavaScriptAction {
    pub fn new(script: String) -> Self {
        Self { script }
    }
}

impl Action for JavaScriptAction {
    fn action_type(&self) -> &'static str {
        "JavaScript"
    }

    fn to_dict(&self) -> PdfResult<PdfDictionaryObject> {
        let mut dict = PdfDictionaryObject::new();
        dict.add("S", Pdf::name(self.action_type()));
        dict.add("JS", Pdf::string(self.script.as_str()));

        Ok(dict)
    }
}

pub struct LaunchAction {
    pub file: String,
    pub new_window: Option<bool>,
}

impl LaunchAction {
    pub fn new(file: String) -> Self {
        Self {
            file,
            new_window: None,
        }
    }

    pub fn with_new_window(mut self, new_window: bool) -> Self {
        self.new_window = Some(new_window);
        self
    }
}

impl Action for LaunchAction {
    fn action_type(&self) -> &'static str {
        "Launch"
    }

    fn to_dict(&self) -> PdfResult<PdfDictionaryObject> {
        let mut dict = PdfDictionaryObject::new();
        dict.add("S", Pdf::name(self.action_type()));

        let mut file_dict = PdfDictionaryObject::new().typed("Filespec");
        file_dict.add("F", Pdf::string(self.file.as_str()));
        dict.add("F", Pdf::dict(file_dict));

        if let Some(new_win) = self.new_window {
            dict.add("NewWindow", Pdf::bool(new_win));
        }

        Ok(dict)
    }
}

pub struct NamedAction {
    pub name: NamedActionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedActionType {
    NextPage,
    PrevPage,
    FirstPage,
    LastPage,
}

impl NamedActionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NamedActionType::NextPage => "NextPage",
            NamedActionType::PrevPage => "PrevPage",
            NamedActionType::FirstPage => "FirstPage",
            NamedActionType::LastPage => "LastPage",
        }
    }
}

impl NamedAction {
    pub fn new(name: NamedActionType) -> Self {
        Self { name }
    }
}

impl Action for NamedAction {
    fn action_type(&self) -> &'static str {
        "Named"
    }

    fn to_dict(&self) -> PdfResult<PdfDictionaryObject> {
        let mut dict = PdfDictionaryObject::new();
        dict.add("S", Pdf::name(self.action_type()));
        dict.add("N", Pdf::name(self.name.as_str()));

        Ok(dict)
    }
}

impl std::fmt::Display for NamedActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Destinations specify a particular view of a PDF page.
#[derive(Debug, Clone)]
pub enum FitDestination {
    XYZ {
        page: usize,
        left: Option<f64>,
        top: Option<f64>,
        zoom: Option<f64>,
    },
    Fit {
        page: usize,
    },
    FitH {
        page: usize,
        top: Option<f64>,
    },
    FitV {
        page: usize,
        left: Option<f64>,
    },
    FitR {
        page: usize,
        rect: Rect,
    },
}

impl FitDestination {
    pub fn xyz(page: usize, left: Option<f64>, top: Option<f64>, zoom: Option<f64>) -> Self {
        Self::XYZ {
            page,
            left,
            top,
            zoom,
        }
    }

    pub fn fit(page: usize) -> Self {
        Self::Fit { page }
    }

    pub fn to_pdf_array(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();

        match self {
            FitDestination::XYZ {
                page,
                left,
                top,
                zoom,
            } => {
                arr.push(Pdf::num(*page));
                arr.push(Pdf::name("XYZ"));
                arr.push(Pdf::num_or_null(*left));
                arr.push(Pdf::num_or_null(*top));
                arr.push(Pdf::num_or_null(*zoom));
            }

            FitDestination::Fit { page } => {
                arr.push(Pdf::num(*page));
                arr.push(Pdf::name("Fit"));
            }

            FitDestination::FitH { page, top } => {
                arr.push(Pdf::num(*page));
                arr.push(Pdf::name("FitH"));
                arr.push(Pdf::num_or_null(*top));
            }

            FitDestination::FitV { page, left } => {
                arr.push(Pdf::num(*page));
                arr.push(Pdf::name("FitV"));
                arr.push(Pdf::num_or_null(*left));
            }

            FitDestination::FitR { page, rect } => {
                arr.push(Pdf::num(*page));
                arr.push(Pdf::name("FitR"));
                arr.push(Pdf::num(rect.x1));
                arr.push(Pdf::num(rect.y1));
                arr.push(Pdf::num(rect.x2));
                arr.push(Pdf::num(rect.y2));
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_action() {
        let action = UriAction::new("https://example.com".to_string());
        let dict = action.to_dict().unwrap();
        assert!(dict.contains_key("S"));
        assert!(dict.contains_key("URI"));
    }

    #[test]
    fn test_goto_action() {
        let dest = FitDestination::xyz(1, Some(0.0), Some(0.0), Some(1.0));
        let action = GoToAction::new(dest);
        let dict = action.to_dict().unwrap();
        assert!(dict.contains_key("S"));
        assert!(dict.contains_key("D"));
    }

    #[test]
    fn test_javascript_action() {
        let action = JavaScriptAction::new("app.alert('Hello');".to_string());
        let dict = action.to_dict().unwrap();
        assert!(dict.contains_key("S"));
        assert!(dict.contains_key("JS"));
    }

    #[test]
    fn test_named_action() {
        let action = NamedAction::new(NamedActionType::NextPage);
        let dict = action.to_dict().unwrap();
        assert!(dict.contains_key("S"));
        assert!(dict.contains_key("N"));
    }
}
