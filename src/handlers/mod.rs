pub mod send_email;
pub mod create_user;
pub mod login;
pub mod forgot_password;
pub mod add_marks;
pub mod edit_marks;

pub mod delete_marks;
// pub mod get_results;
pub mod get_students;
// pub mod publish;
pub use send_email::send_confirmation_email;
pub use create_user::create_user;
pub use login::login_user;
pub use forgot_password::forgot_password;
pub use add_marks::add_marks;
