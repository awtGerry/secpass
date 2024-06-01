/* Purpose:
   This program is a GUI to register an account with a email and password.
   The main objective is to make the user create a secure password that
   has a minimum of 8 characters, at least one uppercase letter, one lowercase
   one number and one special character. Also, the password is going to be
   saved in a database with the password encrypted.

   The program will also have a login page to authenticate the user, to test the
   decryption of the password.
*/

mod passwd;
mod db;
mod mfa;

use crate::passwd::User;

use iced::widget::{
    column,
    row,
    button,
    Container,
    container,
    image,
    checkbox,
    TextInput,
    text,
};
use iced::{Theme, Command, Settings, Element, window, Application};

fn main() -> iced::Result {
    SecPassApp::run(Settings {
        window: window::Settings {
            size: iced::Size {
                width: 800.0,
                height: 600.0,
            },
            resizable: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct SecPassApp {
    pages: Pages,

    name_value: String,
    father_lm_value: String,
    mather_lm_value: String,
    age_value: String,

    email_value: String,
    passwd_value: String,

    msg_color: iced::Color,
    error_msg: String,

    show_password: bool,
    verification_code: String,
}

#[derive(Debug, Default)]
enum Pages {
    #[default]
    Login,
    Register,
    MFA,
}

#[derive(Debug, Clone)]
enum App {
    NameChanged(String),
    FatherLMChanged(String),
    MotherLMChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    AgeChanged(String),
    ToggleShowPassword(bool),
    CodeChanged(String),
    Login,
    Register,
    ChangeToLogin,
    ChangeToRegister,
    SendCode,
}

impl Default for App {
    fn default() -> Self {
        Self::ChangeToLogin
    }
}

impl Application for SecPassApp {
    type Message = App; // Messages that can be sent to the app

    type Theme = Theme; // Custom theme (use default dark for now)
    type Executor = iced::executor::Default; // engine to run async tasks
    type Flags = (); // data passed to the app on startup

    fn new(_flags: Self::Flags) -> (Self, Command<App>) {
        (
            Self {
                pages: Pages::Login,

                name_value: String::new(),
                father_lm_value: String::new(),
                mather_lm_value: String::new(),
                age_value: String::new(),

                email_value: String::new(),
                passwd_value: String::new(),

                msg_color: iced::Color::from_rgb8(210, 15, 57),
                error_msg: String::new(),

                show_password: false,
                verification_code: String::new(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Secure Password")
    }

    fn update(&mut self, message: Self::Message) -> Command<App> {
        match message {
            App::NameChanged(value) => {
                self.name_value = value;
                Command::none()
            }
            App::FatherLMChanged(value) => {
                self.father_lm_value = value;
                Command::none()
            }
            App::MotherLMChanged(value) => {
                self.mather_lm_value = value;
                Command::none()
            }
            App::AgeChanged(value) => {
                self.age_value = value;
                Command::none()
            }
            App::EmailChanged(value) => {
                self.email_value = value;
                // Validate that the email is a valid email
                // This is just a simple validation, it doesn't check if the email is real
                // It just checks if the email has the format of an email
                if !self.email_value.contains('@') || !self.email_value.contains('.') {
                    self.error_msg = String::from("Invalid email");
                } else {
                    self.error_msg = String::new();
                }

                Command::none()
            }
            App::PasswordChanged(value) => {
                self.passwd_value = value;
                Command::none()
            }
            App::ToggleShowPassword(show) => {
                self.show_password = show;
                Command::none()
            }

            App::Login => {
                let red: iced::Color = iced::Color::from_rgb8(210, 15, 57);
                let green: iced::Color = iced::Color::from_rgb8(64, 160, 43);
                let user = User::new(&self.email_value, &self.passwd_value);
                if passwd::login_user(&user.username, &user.password) {
                    self.msg_color = green;
                    self.pages = Pages::MFA;
                } else {
                    self.msg_color = red;
                    self.error_msg = String::from("Invalid username or password");
                }
                Command::none()
            }

            App::Register => {
                let user = User::new(&self.email_value, &self.passwd_value);
                let red: iced::Color = iced::Color::from_rgb8(210, 15, 57);
                let green: iced::Color = iced::Color::from_rgb8(64, 160, 43);
                if let Err(e) = passwd::check_password(&user.password) {
                    match e {
                        passwd::PasswordError::TooShort => {
                            self.msg_color = red;
                            self.error_msg = String::from("Password needs to have at least 8 characters");
                        }
                        passwd::PasswordError::NoUppercase => {
                            self.msg_color = red;
                            self.error_msg = String::from("Password has no uppercase letter");
                        }
                        passwd::PasswordError::NoLowercase => {
                            self.msg_color = red;
                            self.error_msg = String::from("Password has no lowercase letter");
                        }
                        passwd::PasswordError::NoNumber => {
                            self.msg_color = red;
                            self.error_msg = String::from("Password needs to have at least one number");
                        }
                        passwd::PasswordError::NoSpecial => {
                            self.msg_color = red;
                            self.error_msg = String::from("Password needs to have at least one special character");
                        }
                    }
                } else {
                    self.msg_color = green;
                    let age = self.age_value.parse::<u8>().unwrap();
                    let can_register = passwd::register_user(
                        &user.username,
                        &user.password,
                        &self.name_value,
                        &self.father_lm_value,
                        &self.mather_lm_value,
                        age,
                    );
                    self.error_msg = can_register;
                }
                Command::none()
            }
            App::ChangeToLogin => {
                self.pages = Pages::Login;
                self.show_password = false;
                Command::none()
            }
            App::ChangeToRegister => {
                self.pages = Pages::Register;
                self.show_password = false;
                Command::none()
            }
            App::CodeChanged(value) => {
                self.verification_code = value;
                Command::none()
            }
            App::SendCode => {
                self.error_msg = String::from("Welcome aboard!");
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let page = match self.pages {
            Pages::Login => {
                let header = {
                    column![
                        call_image("cat.png", 240),
                        text("Login to your account")
                            .size(24)
                            .width(iced::Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                    ].spacing(10)
                };

                let user_fields = {
                    let user_input = TextInput::new("󰁥  Enter email", &self.email_value)
                        .on_input(App::EmailChanged)
                        .width(480)
                        .padding(10);
                    let passwd_input = TextInput::new("  Enter password", &self.passwd_value)
                        .secure(if self.show_password { false } else { true })
                        .on_input(App::PasswordChanged)
                        .width(480)
                        .padding(10);

                    let check = {
                        checkbox("Show password", self.show_password)
                            .on_toggle(App::ToggleShowPassword)
                    };
                    let error_msg = text(&self.error_msg).size(14).style(self.msg_color);

                    // Separate the inputs with a 20px space between them
                    let inputs = column![ user_input, passwd_input ].spacing(20);
                    // Put the error message below the inputs
                    let msg_container = row![check, error_msg].spacing(10);

                    column![
                        inputs,
                        msg_container
                    ].spacing(10)
                };

                let login_button = {
                    column![
                        button("Login")
                            .on_press_maybe(
                                if self.email_value.is_empty() || self.passwd_value.is_empty() {
                                    None
                                } else {
                                    Some(App::Login)
                                }
                            )
                            .width(480)
                            .padding([10, 20]),

                        button("Don't have an account? Register")
                            .on_press(App::ChangeToRegister)
                            .width(480)
                            .style(iced::theme::Button::Text)
                    ].spacing(5)
                };

                let content = column![
                        header,
                        user_fields,
                        login_button
                ]
                .padding(20)
                .spacing(20)
                .align_items(iced::Alignment::Center);

                container(content).width(iced::Length::Fill).height(iced::Length::Fill).into()
            }

            Pages::Register => {
                let header = {
                    column![
                        text("Register your account")
                            .size(24)
                            .style(iced::Color::from_rgb8(114, 135, 253))
                            .width(iced::Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                    ].spacing(10)
                };

                let user_fields = {
                    let name_input = TextInput::new(" Enter name *", &self.name_value)
                        .on_input(App::NameChanged)
                        .width(380)
                        .padding(10);
                    let father_lastname_input = TextInput::new(" Enter father lastname *", &self.father_lm_value)
                        .on_input(App::FatherLMChanged)
                        .width(380)
                        .padding(10);
                    let mother_lastname_input = TextInput::new(" Enter mother lastname", &self.mather_lm_value)
                        .on_input(App::MotherLMChanged)
                        .width(380)
                        .padding(10);
                    let age_changed = TextInput::new(" Enter your age *", &self.age_value)
                        .on_input(App::AgeChanged)
                        .width(380)
                        .padding(10);
                    let user_input = TextInput::new("󰁥  Enter email *", &self.email_value)
                        .on_input(App::EmailChanged)
                        .width(380)
                        .padding(10);
                    let passwd_input = TextInput::new("  Enter password *", &self.passwd_value)
                        .secure(if self.show_password { false } else { true })
                        .on_input(App::PasswordChanged)
                        .width(380)
                        .padding(10);

                    let check = {
                        checkbox("Show password", self.show_password)
                            .on_toggle(App::ToggleShowPassword)
                    };
                    let error_msg = text(&self.error_msg).size(14).style(self.msg_color);

                    // Separate the inputs with a 20px space between them
                    let inputs = column![
                        name_input,
                        father_lastname_input,
                        mother_lastname_input,
                        age_changed,
                        user_input,
                        passwd_input,
                    ].spacing(20);
                    // Put the error message below the inputs
                    let msg_container = row![check, error_msg].spacing(10);

                    column![
                        inputs,
                        msg_container
                    ].spacing(10)
                };

                let register_button = {
                    column![
                        button("Register")
                            .on_press_maybe(
                                if self.email_value.is_empty() || self.passwd_value.is_empty() ||
                                    self.name_value.is_empty() || self.father_lm_value.is_empty() ||
                                    self.age_value.is_empty()
                                {
                                    None
                                } else {
                                    Some(App::Register)
                                }
                            )
                            .width(480)
                            .padding([10, 20]),

                        button("Already have an account? Login")
                            .on_press(App::ChangeToLogin)
                            .width(480)
                            .style(iced::theme::Button::Text)
                    ].spacing(5)
                };

                let content = column![
                        header,
                        user_fields,
                        register_button
                ]
                .padding(20)
                .spacing(20)
                .align_items(iced::Alignment::Center);

                container(content).width(iced::Length::Fill).height(iced::Length::Fill).into()
            }

            // Multi-factor authentication page
            Pages::MFA => {
                let verify_title = text("Verify your identity")
                    .size(24)
                    .style(iced::Color::from_rgb8(136, 57, 239))
                    .width(iced::Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center);
                let code_input = TextInput::new("Enter code", &self.verification_code)
                    .on_input(|code| App::CodeChanged(code))
                    .width(480)
                    .padding(10);
                let send_button = button("Send code")
                    .on_press(App::SendCode)
                    .width(480)
                    .padding([10, 20]);

                let content = column![
                    verify_title,
                    code_input,
                    send_button
                ]
                .padding(20)
                .spacing(20)
                .align_items(iced::Alignment::Center);
                container(content).width(iced::Length::Fill).height(iced::Length::Fill).into()
            }
        };
        page
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinLatte
    }
}

fn call_image<'a>(file_name: &str, width: u16) -> Container<'a, App> {
    container(
        image(format!("assets/{file_name}")).width(width),
    )
        .width(iced::Length::Fill)
        .center_x()
}
