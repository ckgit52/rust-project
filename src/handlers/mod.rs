pub mod email;
pub mod user;
pub mod auth;
pub mod password;
pub mod marks;
pub mod edit_marks;
// pub mod get_results;
pub mod getStudents;
pub mod publishResults;
pub mod deleteMarks;

pub use email::send_confirmation_email;
pub use user::create_user;
pub use auth::login_user;
pub use password::forgot_password;
pub use marks::add_marks;
