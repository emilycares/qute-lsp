pub struct Route {
    /// A full String specification of the path
    path: String,
    /// Specifies an array of classpaths that are expected. In generic route elements
    ///
    /// Type support order:
    ///   - String, int, long, boolean
    ///   - Enums of our project
    ///   
    parameter_types: Vec<ParameterType>,
    //output_type
}

pub enum ParameterType {
    String,
    Int,
    Long,
    Unknown(String),
}
