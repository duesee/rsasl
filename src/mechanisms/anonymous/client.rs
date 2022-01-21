use std::io::Write;
use crate::mechanism::Authentication;
use crate::property::AnonymousToken;
use crate::SASLError;
use crate::session::{SessionData, StepResult};
use crate::session::Step::Done;

#[derive(Copy, Clone, Debug)]
pub struct Anonymous;

impl Authentication for Anonymous {
    fn step(&mut self, session: &mut SessionData, _input: Option<&[u8]>, writer: &mut dyn Write)
        -> StepResult
    {
        if let Some(token) = session.get_property_or_callback::<AnonymousToken>()? {
            let buf = token.as_bytes();
            writer.write_all(buf)?;
            Ok(Done(Some(buf.len())))
        } else {
            Err(SASLError::no_property::<AnonymousToken>())
        }
    }
}