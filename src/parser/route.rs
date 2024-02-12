use std::{
    fs,
    path::{Path, PathBuf},
};

use dashmap::DashMap;
use tower_lsp::lsp_types::{CompletionItem, Location, Url};
use tree_sitter::{Node, Parser};

use crate::{extraction::to_lsp_position, file_utils::find_files};

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub implementation: Option<Location>,
    pub method: HttpMethod,
    /// A full String specification of the path
    pub path: String,
    /// Specifies an array of classpaths that are expected. In generic route elements
    pub parameters: Vec<Parameter>,
    pub produces_type: MediaType,
}

impl Route {
    pub fn append_to_base(mut self, other: Self) -> Self {
        self.method = other.method;
        if self.path.ends_with('/') {
            self.path = self.path[0..self.path.len() - 1].to_string() + other.path.as_str();
        } else {
            self.path += &other.path;
        }
        self.parameters.extend(other.parameters);
        self.produces_type = other.produces_type;
        self
    }
}

impl Default for Route {
    fn default() -> Self {
        Self {
            implementation: None,
            method: HttpMethod::Get,
            path: String::new(),
            parameters: vec![],
            produces_type: MediaType::TextPlain,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaType {
    ///A String constant representing "application/atom+xml" media type.
    ApplicationAtomXml,
    ///A String constant representing "application/x-www-form-urlencoded" media type.
    ApplicationFormUrlencoded,
    ///A String constant representing "application/json" media type.
    ApplicationJson,
    ///String representation of "application/json-patch+json" media type..
    ApplicationJsonPatchJson,
    ///A String constant representing "application/octet-stream" media type.
    ApplicationOctetStream,
    ///A String constant representing "application/svg+xml" media type.
    ApplicationSvgXml,
    ///A String constant representing "application/xhtml+xml" media type.
    ApplicationXhtmlXml,
    ///A String constant representing "application/xml" media type.
    ApplicationXml,
    ///The media type charset parameter name.
    ///A String constant representing "multipart/form-data" media type.
    MultipartFormData,
    ///String representation of Server sent events media type.
    ServerSentEvents,
    ///A String constant representing "text/html" media type.
    TextHtml,
    ///A String constant representing "text/plain" media type.
    TextPlain,
    ///A String constant representing "text/xml" media type.
    TextXml,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Options,
    Trace,
    Patch,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    name: String,
    java_type: ParameterType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterType {
    String,
    Int,
    Long,
    Unknown(String),
}

pub fn completion(
    route_map: &DashMap<String, Route>,
    line: String,
    char_pos: usize,
) -> Vec<CompletionItem> {
    let content = &line;
    let mut parser = Parser::new();
    let language = pepegsitter::java::language();
    //let language = tree_sitter_html::language();
    parser
        .set_language(language)
        .expect("Error loading html grammar");
    let Some(tree) = parser.parse(&content, None) else {
        return vec![];
    };
    let mut cursor = tree.walk();
    cursor.goto_first_child_for_point(tree_sitter::Point {
        row: 0,
        column: char_pos,
    });
    let string_literal_node = cursor.node();
    if string_literal_node.kind() != "string_literal" {
        dbg!("not string_literal");
        dbg!(string_literal_node.kind());
        dbg!(string_literal_node.utf8_text(content.as_bytes()));
        return vec![];
    }
    let param_name = get_param_name(string_literal_node, content);
    if !can_complet_path_for_param_name(param_name) {
        dbg!("not can_complet_path_for_param_name");
        return vec![];
    }
    return route_map
        .iter()
        .map(|r| {
            CompletionItem::new_simple(
                r.key().to_string(), /* + optional_close*/
                "A quarkus route".to_string(),
            )
        })
        .collect::<Vec<_>>();
}

fn can_complet_path_for_param_name(param_name: Option<String>) -> bool {
    let Some(param_name) = param_name else {
        return false;
    };
    match param_name.as_str() {
        "hx-get" => true,
        _ => false,
    }
}

fn get_param_name<'a, 'b>(node: Node, content: &'a str) -> Option<String> {
    let Some(node) = node.prev_sibling() else {
        return None;
    };
    let Some(node) = node.prev_sibling() else {
        return None;
    };
    let mut tag_name = vec![];
    if let Ok(n) = node.utf8_text(content.as_bytes()) {
        tag_name.push(n);
    }
    let Some(node) = node.prev_sibling() else {
        return None;
    };
    if let Ok(n) = node.utf8_text(content.as_bytes()) {
        tag_name.push(n);
    }
    let Some(node) = node.prev_sibling() else {
        return None;
    };
    if let Ok(n) = node.utf8_text(content.as_bytes()) {
        tag_name.push(n);
    }
    let s = tag_name.clone().into_iter().rev().collect::<String>();
    if s.contains(' ') {
        let Some((_, b)) = s.rsplit_once(' ') else {
            return None;
        };
        let b = b.to_string();

        return Some(b);
    } else {
        return Some(s.to_string());
    }
}

pub fn scan_routes() -> Vec<Route> {
    let template_folder = "./src/main/java/";
    let path = Path::new(&template_folder);
    if let Ok(files) = find_files(path) {
        return files
            .into_iter()
            .flat_map(|p| {
                if let Ok(con) = fs::read_to_string(p.clone()) {
                    if let Some(filename) = p.to_str() {
                        if let Some(file_path) =
                            std::fs::canonicalize::<PathBuf>(filename.into()).ok()
                        {
                            return Some(analyse_file(file_path, &con));
                        }
                    }
                }
                None
            })
            .flatten()
            .collect();
    }

    vec![]
}

pub fn analyse_file(file_path: PathBuf, content: &str) -> Vec<Route> {
    let mut out = vec![];
    let mut parser = Parser::new();
    let language = pepegsitter::java::language();
    parser
        .set_language(language)
        .expect("Error loading java grammar");
    let Some(tree) = parser.parse(&content, None) else {
        return vec![];
    };
    let mut cursor = tree.walk();
    cursor.goto_first_child();
    skip_head(&mut cursor);
    out.extend(handel_classes(file_path, content, &mut cursor));
    out
}

fn analyse_class<'a, 'b>(
    file_path: PathBuf,
    content: &'a str,
    cursor: &mut tree_sitter::TreeCursor<'a>,
) -> Vec<Route> {
    let mut out = vec![];
    cursor.goto_first_child();
    // analyse annotations at class level
    let Some(base_route) = analyse_modifiers(content, cursor) else {
        return vec![];
    };
    cursor.goto_parent();
    cursor.goto_next_sibling();
    cursor.goto_next_sibling();
    cursor.goto_next_sibling();
    if cursor.node().kind() == "superclass" {
        cursor.goto_next_sibling();
    }
    if cursor.node().kind() == "super_interfaces" {
        cursor.goto_next_sibling();
    }
    cursor.goto_first_child();
    out.extend(analyse_fields(base_route, file_path, content, cursor));
    cursor.goto_parent();

    cursor.goto_parent();
    out
}

fn analyse_fields<'a, 'b>(
    base_route: Route,
    file_path: PathBuf,
    content: &'a str,
    cursor: &mut tree_sitter::TreeCursor<'a>,
) -> Vec<Route> {
    let mut out: Vec<Route> = vec![];

    match cursor.node().kind() {
        "method_declaration" => {
            if let Some(r) = analyse_method(&base_route, file_path.clone(), content, cursor) {
                out.push(r);
            }
        }
        "field_declaration" => (),
        _ => (),
    }

    if cursor.goto_next_sibling() {
        out.extend(analyse_fields(base_route, file_path, content, cursor));
    }
    out
}

fn analyse_method<'a, 'b>(
    base_route: &Route,
    file_path: PathBuf,
    content: &'a str,
    cursor: &mut tree_sitter::TreeCursor<'a>,
) -> Option<Route> {
    cursor.goto_first_child();
    let Some(r) = analyse_modifiers(content, cursor) else {
        return None;
    };
    let mut route = base_route.clone().append_to_base(r);
    cursor.goto_parent();
    cursor.goto_next_sibling();
    cursor.goto_next_sibling();
    let method_position = cursor.node().start_position();
    route.implementation = match Url::from_file_path(file_path) {
        Ok(url) => Some(Location::new(
            url,
            tower_lsp::lsp_types::Range {
                start: to_lsp_position(method_position),
                end: to_lsp_position(method_position),
            },
        )),
        Err(_) => None,
    };
    cursor.goto_next_sibling();
    analyse_method_parameters(&mut route, content, cursor);
    cursor.goto_parent();

    return Some(route);
}

fn analyse_method_parameters<'a, 'b>(
    route: &mut Route,
    content: &'a str,
    cursor: &mut tree_sitter::TreeCursor<'a>,
) {
    cursor.goto_first_child();

    while cursor.goto_next_sibling() {
        let mut name: &str = "";
        let mut java_type = ParameterType::Unknown("".to_owned());
        if cursor.node().kind() == "formal_parameter" {
            cursor.goto_first_child();
            if cursor.node().kind() == "modifiers" {
                cursor.goto_first_child();
                if cursor.node().kind() == "annotation" {
                    cursor.goto_first_child();
                    cursor.goto_next_sibling();
                    if let Ok(annotation_name) = cursor.node().utf8_text(content.as_bytes()) {
                        match annotation_name {
                            "PathParam" => {
                                cursor.goto_next_sibling();
                                cursor.goto_first_child();
                                cursor.goto_next_sibling();
                                cursor.goto_first_child();
                                cursor.goto_next_sibling();
                                if let Ok(parameter_name) =
                                    cursor.node().utf8_text(content.as_bytes())
                                {
                                    name = parameter_name;
                                }
                                cursor.goto_parent();
                                cursor.goto_parent();
                            }
                            _ => (),
                        }
                    }

                    cursor.goto_parent();
                }
                cursor.goto_parent();
            }
            cursor.goto_next_sibling();
            if let Ok(ty) = cursor.node().utf8_text(content.as_bytes()) {
                if let Some(ty) = parse_java_type_for_param(ty) {
                    java_type = ty;
                }
            }
            for c in &mut route.parameters {
                if c.name == name {
                    c.java_type = java_type.clone();
                };
            }
            cursor.goto_parent();
        }
    }

    cursor.goto_parent();
}

fn analyse_modifiers<'a, 'b>(
    content: &'a str,
    cursor: &mut tree_sitter::TreeCursor<'a>,
) -> Option<Route> {
    if cursor.node().kind() != "modifiers" {
        return None;
    }
    cursor.goto_first_child();
    let mut out = Route::default();

    if analyse_modifier(&mut out, content, cursor) {
        return Some(out);
    }
    None
}

fn analyse_modifier<'a, 'b>(
    route: &mut Route,
    content: &'a str,
    cursor: &mut tree_sitter::TreeCursor<'a>,
) -> bool {
    //dbg!(cursor.node().utf8_text(content.as_bytes()).unwrap());
    let mut changed = false;
    match cursor.node().kind() {
        "annotation" => {
            cursor.goto_first_child();
            cursor.goto_next_sibling();
            if let Ok(name) = cursor.node().utf8_text(content.as_bytes()) {
                match name {
                    "Path" => {
                        cursor.goto_next_sibling();
                        cursor.goto_first_child();
                        cursor.goto_next_sibling();
                        cursor.goto_first_child();
                        cursor.goto_next_sibling();
                        if let Ok(path) = cursor.node().utf8_text(content.as_bytes()) {
                            route.path += path;
                            changed = true;
                            route.parameters.extend(initialise_paramters(path))
                        }
                        cursor.goto_parent();
                        cursor.goto_parent();
                    }
                    "Produces" => {
                        cursor.goto_next_sibling();
                        cursor.goto_first_child();
                        cursor.goto_next_sibling();
                        cursor.goto_first_child();
                        cursor.goto_next_sibling();
                        cursor.goto_next_sibling();
                        if let Ok(produces) = cursor.node().utf8_text(content.as_bytes()) {
                            if let Some(media_type) = parse_jakarta_media_type(produces) {
                                route.produces_type = media_type;
                                changed = true;
                            }
                        }
                        cursor.goto_parent();
                        cursor.goto_parent();
                    }
                    _ => (),
                }
            }
            cursor.goto_parent();
        }
        "marker_annotation" => {
            cursor.goto_first_child();
            cursor.goto_next_sibling();
            if let Ok(annotation_name) = cursor.node().utf8_text(content.as_bytes()) {
                if let Some(jakarta_method) =
                    parse_jakarta_http_method_annotation_name(annotation_name)
                {
                    route.method = jakarta_method;
                    changed = true;
                }
            }
            cursor.goto_parent();
        }
        _ => (),
    }

    if cursor.goto_next_sibling() {
        let next = analyse_modifier(route, content, cursor);
        if next {
            changed = true;
        }
    }

    changed
}

fn initialise_paramters<'a, 'b>(path: &'a str) -> Vec<Parameter> {
    let mut out = vec![];
    let mut name = String::new();
    for char in path.chars() {
        match char {
            '{' => {
                name = String::new();
            }
            '}' => {
                out.push(Parameter {
                    name: name.to_owned(),
                    java_type: ParameterType::Unknown("".to_owned()),
                });
            }
            _ => {
                name.push(char);
            }
        }
    }

    out
}

fn parse_jakarta_media_type(annotation_name: &str) -> Option<MediaType> {
    match annotation_name {
        "APPLICATION_XML" => Some(MediaType::ApplicationXml),
        "APPLICATION_ATOM_XML" => Some(MediaType::ApplicationAtomXml),
        "APPLICATION_XHTML_XML" => Some(MediaType::ApplicationXhtmlXml),
        "APPLICATION_SVG_XML" => Some(MediaType::ApplicationSvgXml),
        "APPLICATION_JSON" => Some(MediaType::ApplicationJson),
        "APPLICATION_FORM_URLENCODED" => Some(MediaType::ApplicationFormUrlencoded),
        "MULTIPART_FORM_DATA" => Some(MediaType::MultipartFormData),
        "APPLICATION_OCTET_STREAM" => Some(MediaType::ApplicationOctetStream),
        "TEXT_PLAIN" => Some(MediaType::TextPlain),
        "TEXT_XML" => Some(MediaType::TextXml),
        "TEXT_HTML" => Some(MediaType::TextHtml),
        "SERVER_SENT_EVENTS" => Some(MediaType::ServerSentEvents),
        "APPLICATION_JSON_PATCH_JSON" => Some(MediaType::ApplicationJsonPatchJson),
        _ => None,
    }
}

fn parse_jakarta_http_method_annotation_name(annotation_name: &str) -> Option<HttpMethod> {
    match annotation_name {
        "GET" => Some(HttpMethod::Get),
        "HEAD" => Some(HttpMethod::Head),
        "POST" => Some(HttpMethod::Post),
        "PUT" => Some(HttpMethod::Put),
        "DELETE" => Some(HttpMethod::Delete),
        "OPTIONS" => Some(HttpMethod::Options),
        "TRACE" => Some(HttpMethod::Trace),
        "PATCH" => Some(HttpMethod::Patch),
        _ => None,
    }
}
fn parse_java_type_for_param(ty: &str) -> Option<ParameterType> {
    match ty {
        "String" => Some(ParameterType::String),
        "int" => Some(ParameterType::Int),
        "long" => Some(ParameterType::Long),
        _ => Some(ParameterType::Unknown(ty.to_string())),
    }
}

fn skip_head(cursor: &mut tree_sitter::TreeCursor<'_>) {
    if cursor.node().kind() == "package_declaration" || cursor.node().kind() == "import_declaration"
    {
        cursor.goto_next_sibling();
        skip_head(cursor);
    }
}
fn handel_classes<'a, 'b>(
    file_path: PathBuf,
    content: &'a str,
    cursor: &mut tree_sitter::TreeCursor<'a>,
) -> Vec<Route> {
    let mut out = vec![];
    if cursor.node().kind() == "class_declaration" {
        out.extend(analyse_class(file_path.clone().clone(), content, cursor));
        // when there is a sibling then also scann that class
        if cursor.goto_next_sibling() {
            out.extend(handel_classes(file_path, content, cursor));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::parser::route::{
        analyse_file, HttpMethod, MediaType, Parameter, ParameterType, Route,
    };
    use dashmap::DashMap;
    use pretty_assertions::assert_eq;
    use tower_lsp::lsp_types::CompletionItem;

    use super::completion;

    #[test]
    fn analyse_file_test() {
        static FILE_CONTENT: &str = include_str!("../../test/BasicResource.java");
        let out = analyse_file("".into(), FILE_CONTENT);
        assert_eq!(
            out,
            vec![
                Route {
                    implementation: None,
                    method: HttpMethod::Get,
                    path: "/hello".to_string(),
                    parameters: vec![],
                    produces_type: MediaType::TextHtml
                },
                Route {
                    implementation: None,
                    method: HttpMethod::Get,
                    path: "/hello/customer/{name}".to_string(),
                    parameters: vec![Parameter {
                        name: "name".to_owned(),
                        java_type: ParameterType::String
                    }],
                    produces_type: MediaType::TextHtml
                },
                Route {
                    implementation: None,
                    method: HttpMethod::Put,
                    path: "/hello/customer/{name}/{sufix}".to_string(),
                    parameters: vec![
                        Parameter {
                            name: "name".to_string(),
                            java_type: ParameterType::String,
                        },
                        Parameter {
                            name: "sufix".to_string(),
                            java_type: ParameterType::Int,
                        },
                    ],
                    produces_type: MediaType::ApplicationJson,
                },
            ]
        )
    }

    #[test]
    fn completion_basic() {
        let dm = DashMap::new();
        dm.insert("/start".to_string(), Route::default());
        let out = completion(&dm, "<button hx-get=\"/sel\" hx-trigger=\"click\" hx-target=\"#selectStyle\" hx-swap=\"outerHTML\"></button>".to_string(), 21);
        assert_eq!(
            out,
            vec![CompletionItem {
                label: "/start".to_string(),
                detail: Some("A quarkus route".to_string()),
                ..CompletionItem::default()
            }]
        )
    }
    #[test]
    fn completion_basic_not() {
        let dm = DashMap::new();
        dm.insert("".to_string(), Route::default());
        let out = completion(&dm, "<button hx-get=\"/sel\" hx-trigger=\"click\" hx-target=\"#selectStyle\" hx-swap=\"outerHTML\"></button>".to_string(), 63);
        assert_eq!(out, vec![])
    }
}
