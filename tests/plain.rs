use std::io;
use std::io::Cursor;
use std::sync::Arc;
use rsasl::callback::Callback;
use rsasl::error::SASLError;
use rsasl::mechname::Mechname;
use rsasl::property::{AuthId, Password};
use rsasl::SASL;
use rsasl::session::SessionData;
use rsasl::session::Step::{Done, NeedsMore};
use rsasl::validate::{Validation, validations};

#[test]
fn plain_client() {
    let sasl = SASL::new();
    let mut session = sasl.client_start(Mechname::new(b"PLAIN").unwrap())
        .unwrap();

    let username = "testuser".to_string();
    assert_eq!(username.len(), 8);
    let password = "secret".to_string();
    assert_eq!(password.len(), 6);

    session.set_property::<AuthId>(Box::new(username));
    session.set_property::<Password>(Box::new(password));

    let mut out = Cursor::new(Vec::new());

    // Do an authentication step. In a PLAIN exchange there is only one step, with no data.
    let data: Option<Vec<u8>> = None;
    let step_result = session.step(data, &mut out).unwrap();

    match step_result {
        Done(Some(len)) => {
            assert_eq!(len, 1 + 8 + 1 + 6);
            let buffer = &out.into_inner()[0..len];
            // (1) "\0" + (8) "testuser" + (1) "\0" + (6) "secret"
            let (name, pass) = buffer.split_at(9);
            assert_eq!(name[0], 0);
            assert_eq!(name, b"\0testuser");
            assert_eq!(pass[0], 0);
            assert_eq!(pass, b"\0secret");
            return;
        },
        Done(None) => panic!("PLAIN exchange produced no output"),
        NeedsMore(_) => panic!("PLAIN exchange took more than one step"),
    }
}

#[test]
fn plain_server() {
    struct CB;
    impl Callback for CB {
        fn validate(&self, session: &mut SessionData, validation: Validation, mechanism: &Mechname)
            -> Result<(), SASLError>
        {
            match validation {
                validations::SIMPLE => {
                    let username = session.get_property::<AuthId>()?;
                    let password = session.get_property::<Password>()?;

                    if username.as_str() == "testuser" && password.as_str() == "secret" {
                        Ok(())
                    } else {
                        Err(SASLError::AuthenticationFailure { reason: "bad username/password"})
                    }
                },
                _ => Err(SASLError::NoValidate { validation }),
            }
        }
    }

    let mut prov = SASL::new();
    prov.install_callback(Arc::new(CB));
    let mut session = prov.server_start(Mechname::new(b"PLAIN").unwrap()).unwrap();

    let mut out = Cursor::new(Vec::new());

    match session.step(Some(b"\0testuser\0secret"), &mut out).unwrap() {
        Done(Some(_)) => {
            panic!("PLAIN mechanism wants to return data: {:?}", &out.into_inner()[..]);
        }
        Done(None) => {}
        NeedsMore(_) => panic!("PLAIN exchange took more than one step"),
    }

    let mut session = prov.server_start(Mechname::new(b"PLAIN").unwrap()).unwrap();
    session.step(Some(b"\0testuser\0badpass"), &mut out).unwrap_err();
}