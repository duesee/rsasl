#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

pub mod consts;
pub mod gsasl;

pub mod anonymous {
    pub mod client;
    pub mod mechinfo;
    pub mod server;
}

// mod anonymous
pub mod cram_md5 {
    pub mod challenge;
    pub mod client;
    pub mod digest;
    pub mod mechinfo;
    pub mod server;
}

// mod cram_md5
pub mod digest_md5 {
    pub mod client;
    pub mod digesthmac;
    pub mod free;
    pub mod getsubopt;
    pub mod mechinfo;
    pub mod nonascii;
    pub mod parser;
    pub mod printer;
    pub mod qop;
    pub mod server;
    pub mod session;
    pub mod validate;
}

// mod digest_md5
pub mod external {
    pub mod client;
    pub mod mechinfo;
    pub mod server;
}

// mod external
pub mod gl {
    pub mod c_ctype;
    pub mod free;
    pub mod gc_gnulib;
    pub mod gc_pbkdf2;
    pub mod hmac_md5;
    pub mod hmac_sha1;
    pub mod hmac_sha256;
    pub mod md5;
    pub mod md5_stream;
    pub mod memxor;
    pub mod sha1;
    pub mod sha1_stream;
    pub mod sha256;
    pub mod sha256_stream;
}

// mod gl
pub mod login {
    pub mod client;
    pub mod mechinfo;
    pub mod server;
}

// mod login
pub mod openid20 {
    pub mod client;
    pub mod mechinfo;
    pub mod server;
}

// mod openid20
pub mod plain {
    pub mod client;
    pub mod mechinfo;
    pub mod server;
}

// mod plain
pub mod saml20 {
    pub mod client;
    pub mod mechinfo;
    pub mod server;
}// mod saml20

pub mod scram {
    pub mod client;
    pub mod mechinfo;
    pub mod parser;
    pub mod printer;
    pub mod server;
    pub mod tokens;
    pub mod tools;
    pub mod validate;
}

// mod scram
pub mod securid {
    pub mod client;
    pub mod mechinfo;
    pub mod server;
} // mod securid

pub mod base64;
pub mod callback;
pub mod crypto;
pub mod done;
pub mod error;
pub mod free;
pub mod init;
pub mod listmech;
pub mod md5pwd;
pub mod mechname;
pub mod mechtools;
pub mod property;
pub mod register;
pub mod saslprep;
pub mod suggest;
pub mod supportp;
pub mod version;
pub mod xcode;
pub mod xfinish;
pub mod xstart;
pub mod xstep;
pub mod gc;

