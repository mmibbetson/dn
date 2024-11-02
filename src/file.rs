// idempotent creation of default directory ~/dnotes
// is created if and only if there is no directory passed in as an arg or in the config file

use std::{env, path::PathBuf};
