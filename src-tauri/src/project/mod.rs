pub mod truth_file;
pub mod facts;
pub mod validation;
pub mod constants;

pub use facts::{answer_question, create_project_from_scan, update_fact_status};
pub use truth_file::*;
pub use validation::validate_project;
