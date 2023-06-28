use std::collections::HashMap;
use std::hash::Hash;
use std::io::ErrorKind;
use std::path::Path;
use strum::IntoEnumIterator;
//use strum::EnumIter;

//Implement this on your own struct, and derive Debug, strum::EnumIter, PartialEq, Clone, Eq, Hash
pub trait TokenDetails {
    fn get_token_path(token_type: &Self) -> String;
    fn get_token_name(token_type: &Self) -> String;
}

/// Describe each variant of an enum very briefly.
trait EnumDescription {
    fn enum_description(&self) -> String;
}

impl EnumDescription for ErrorKind {
    fn enum_description(&self) -> String {
        match self {
            ErrorKind::NotFound => "not found",
            ErrorKind::PermissionDenied => "permission denied",
            ErrorKind::NotConnected => "not connected to network",
            ErrorKind::InvalidInput => "invalid input",
            ErrorKind::InvalidData => "invalid data",
            ErrorKind::TimedOut => "timed out",
            ErrorKind::Interrupted => "interrupted",
            ErrorKind::Unsupported => "unsupported on this platform",
            ErrorKind::UnexpectedEof => "unexpected end of file",
            ErrorKind::OutOfMemory => "out of memory",
            ErrorKind::ConnectionRefused => "network connection refused",
            ErrorKind::ConnectionReset => "network connection reset",
            ErrorKind::ConnectionAborted => "network connection aborted",
            ErrorKind::AddrInUse => "network address already in use",
            ErrorKind::AddrNotAvailable => "network address not available",
            ErrorKind::BrokenPipe => "network pipe was broken",
            ErrorKind::AlreadyExists => "resource already exists",
            ErrorKind::WouldBlock => "operation requested to not block but would",
            ErrorKind::WriteZero => "nothing written",
            ErrorKind::Other => "other",
            _ => "unspecified",
        }
        .to_string()
    }
}

pub struct TokenLoader<
    TToken: TokenDetails + Sized + IntoEnumIterator + Eq + Hash + std::fmt::Debug,
> {
    tokens: HashMap<TToken, String>,
}

impl<TToken: TokenDetails + Sized + IntoEnumIterator + Eq + Hash + std::fmt::Debug>
    TokenLoader<TToken>
{
    ///Give this your token enum. Panics at runtime if not all tokens cannot be loaded
    pub fn new() -> TokenLoader<TToken> {
        let tokens = {
            let mut tokens = Vec::new();

            for token in TToken::iter() {
                let path = TToken::get_token_path(&token);
                let string = std::fs::read_to_string(&path);
                let string = if let Ok(string) = string {
                    let s = string.trim().to_owned();
                    println!("Loaded token '{}'", TToken::get_token_name(&token));
                    s
                } else {
                    let error = string.err().unwrap();

                    let path_message: String = 'path: {
                        let path_struct = Path::new(&path);

                        {
                            let cannon = path_struct.canonicalize();
                            if let Ok(c) = cannon {
                                break 'path format!("valid path at '{:#?}'", c);
                            }
                        }

                        {
                            let parent = path_struct.parent();
                            if let Some(p) = parent {
                                break 'path format!(
                                    "invalid target inside of valid directory '{:#?}'",
                                    p
                                );
                            }
                        }

                        break 'path format!("invalid path given as {}", &path);
                    };

                    let error_description = error.kind().enum_description();

                    panic!("Failed because of reason '{}'. token name '{}', token path '{}'.\nRaw error: {:#?}. Path description: {}", error_description, TToken::get_token_name(&token), &path, error, path_message);
                };

                tokens.push((token, string))
            }
            tokens
        };

        TokenLoader {
            tokens: {
                let mut map = HashMap::with_capacity(tokens.len());
                for (key, token_value) in tokens.into_iter() {
                    map.insert(key, token_value);
                }

                map
            },
        }
    }
}

impl<TToken: TokenDetails + Sized + IntoEnumIterator + Eq + Hash + std::fmt::Debug>
    TokenLoader<TToken>
{
    /// Get the token and handle the error case
    pub fn get(&self, token: &TToken) -> Option<String> {
        self.tokens.get(token).map(|v| v.to_string()) //Changes Option<T> to Option<T.to_string()>
    }

    /// Get the token from the hashmap and unwrap the result. Could panic, but hopefully it never will.
    pub fn get_unwrap(&self, token: &TToken) -> String {
        let value = self.get(token);
        match value {
            Some(v) => v,
            None => {
                panic!(
                    "No token key found for token key'{:#?}' This should not have happened!",
                    token
                )
            }
        }
    }
}
