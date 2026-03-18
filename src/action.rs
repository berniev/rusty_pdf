//! Action framework for interactive PDF behaviors.
//!
//! Actions define behaviors that can be triggered by user interactions, such as
//! clicking links, opening documents, or interacting with form fields.

use crate::{ArrayObject, BooleanObject, DictionaryObject, NameObject, PdfResult, StringObject, util::Rect};

/// Actions specify responses to various events in PDF documents, such as
/// user interactions with annotations or form fields.
pub trait Action {
    /// Get the action type (URI, GoTo, JavaScript, etc.)
    fn action_type(&self) -> &'static str;

    /// Convert this action to a PDF dictionary object.
    fn to_dict(&self) -> PdfResult<DictionaryObject>;
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

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);
        dict.set("S", NameObject::build(self.action_type()));
        dict.set("URI", StringObject::build(&self.uri));

        if self.is_map {
            dict.set("IsMap", BooleanObject::build(true));
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

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);
        dict.set("S", NameObject::build(self.action_type()));
        dict.set("D", self.destination.clone().build());
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

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);
        dict.set("S", NameObject::build(self.action_type()));
        dict.set("JS", StringObject::build(self.script.clone()));
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

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);
        dict.set("S", NameObject::build(self.action_type()));

        let mut file_dict = DictionaryObject::new(None);
        file_dict.set("Type", NameObject::build("Filespec"));
        file_dict.set("F", StringObject::build(self.file.clone()));
        dict.set("F", DictionaryObject::build(file_dict.values));

        if let Some(new_win) = self.new_window {
            dict.set("NewWindow", BooleanObject::build(new_win));
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

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);
        dict.set("S", NameObject::build(self.action_type()));
        dict.set("N", NameObject::build(self.name.as_str()));
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
    Fit { page: usize },
    FitH { page: usize, top: Option<f64> },
    FitV { page: usize, left: Option<f64> },
    FitR { page: usize, rect: Rect },
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

    pub fn build(self) -> std::rc::Rc<dyn crate::PdfObject> {
        std::rc::Rc::new(self.to_array())
    }

    pub fn to_array(&self) -> ArrayObject {
        let mut arr = ArrayObject::new(None);

        match self {
            FitDestination::XYZ {
                page,
                left,
                top,
                zoom,
            } => {
                arr.push_number(*page as i64);
                arr.push_name("XYZ");
                arr.push_optional_real(*left);
                arr.push_optional_real(*top);
                arr.push_optional_real(*zoom);
            }
            FitDestination::Fit { page } => {
                arr.push_number(*page as i64);
                arr.push_name("Fit");
            }
            FitDestination::FitH { page, top } => {
                arr.push_number(*page as i64);
                arr.push_name("FitH");
                arr.push_optional_real(*top);
            }
            FitDestination::FitV { page, left } => {
                arr.push_number(*page as i64);
                arr.push_name("FitV");
                arr.push_optional_real(*left);
            }
            FitDestination::FitR { page, rect } => {
                arr.push_number(*page as i64);
                arr.push_name("FitR");
                arr.push_real(rect.x1);
                arr.push_real(rect.y1);
                arr.push_real(rect.x2);
                arr.push_real(rect.y2);
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

    #[test]
    fn test_destination_xyz() {
        let dest = FitDestination::xyz(0, Some(100.0), Some(200.0), None);
        let arr = ArrayObject::from_destination(dest);
        assert_eq!(arr.values.len(), 5); // page, /XYZ, left, top, zoom
    }

    #[test]
    fn test_destination_fit() {
        let dest = FitDestination::fit(2);
        let arr = ArrayObject::from_destination(dest);
        assert_eq!(arr.values.len(), 2); // page, /Fit
    }
}
