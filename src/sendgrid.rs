use sendgrid_flows::{Email, send_email};
use slack_flows::{listen_to_channel};

#[no_mangle]
pub fn run() {
    listen_to_channel("wasmhaiku", "general", |sm| {
        let email = Email {
            to: vec![String::from("vivian.xiage@gamil.com")],
            subject: String::from("Hi"),
            content: sm.text
        };
        send_email("vivian@secondstate.io", email);
    });
}
