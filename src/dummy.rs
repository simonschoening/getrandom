use crate::Error;

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    Err(Error::UNSUPPORTED)
}