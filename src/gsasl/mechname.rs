use crate::Session;

/* *
 * gsasl_mechanism_name:
 * @sctx: libgsasl session handle.
 *
 * This function returns the name of the SASL mechanism used in the
 * session.  The pointer must not be deallocated by the caller.
 *
 * Return value: Returns a zero terminated character array with the
 *   name of the SASL mechanism, or NULL if not known.
 *
 * Since: 0.2.28
 **/
pub unsafe fn gsasl_mechanism_name(sctx: &Session) -> &'static str {
    return (*(*sctx).mech).name;
}
