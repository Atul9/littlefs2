//! Path manipulation

use core::convert::AsRef;
use core::{cmp, fmt};

use generic_array::{typenum::marker_traits::Unsigned as _, ArrayLength, GenericArray};

use crate::driver;

pub struct Filename<S>(GenericArray<u8, S::FILENAME_MAX_PLUS_ONE>)
where
    S: driver::Storage,
    <S as driver::Storage>::FILENAME_MAX_PLUS_ONE: ArrayLength<u8>;

// to compare filename
impl<S> cmp::PartialEq for Filename<S>
where
    S: driver::Storage,
    <S as driver::Storage>::FILENAME_MAX_PLUS_ONE: ArrayLength<u8>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// to make `DirEntry` Clone
impl<S> Clone for Filename<S>
where
    S: driver::Storage,
    <S as driver::Storage>::FILENAME_MAX_PLUS_ONE: ArrayLength<u8>,
{
    fn clone(&self) -> Self {
        let mut cloned: GenericArray<u8, S::FILENAME_MAX_PLUS_ONE> = Default::default();
        cloned.copy_from_slice(&self.0);
        Filename(cloned)
    }
}

// to make `Metadata` Debug
impl<S> fmt::Debug for Filename<S>
where
    S: driver::Storage,
    <S as driver::Storage>::FILENAME_MAX_PLUS_ONE: ArrayLength<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<S> Filename<S>
where
    S: driver::Storage,
    <S as driver::Storage>::FILENAME_MAX_PLUS_ONE: ArrayLength<u8>,
{
    /// Silently truncates to maximum configured path length
    // pub fn new<F: AsRef<[u8]> + ?Sized>(f: &F) -> Self {
    pub fn new(f: &[u8]) -> Self {
        let mut padded_filename: GenericArray<u8, S::FILENAME_MAX_PLUS_ONE> = Default::default();
        let name_max = <S as driver::Storage>::FILENAME_MAX_PLUS_ONE::to_usize();
        // let len = cmp::min(name_max - 1, f.as_ref().len());
        // padded_filename[..len].copy_from_slice(&f.as_ref()[..len]);
        let len = cmp::min(name_max - 1, f.len());
        padded_filename[..len].copy_from_slice(&f[..len]);
        Filename(padded_filename)
    }
}

/// A slice of a specification of the location of a [`File`](../fs/struct.File.html).
///
/// This module is rather incomplete, compared to `std::path`.
pub struct Path<S>(GenericArray<u8, S::PATH_MAX_PLUS_ONE>)
where
    S: driver::Storage,
    <S as driver::Storage>::PATH_MAX_PLUS_ONE: ArrayLength<u8>;

// to make `Metadata` Clone
impl<S> Clone for Path<S>
where
    S: driver::Storage,
    <S as driver::Storage>::PATH_MAX_PLUS_ONE: ArrayLength<u8>,
{
    fn clone(&self) -> Self {
        let mut cloned: GenericArray<u8, S::PATH_MAX_PLUS_ONE> = Default::default();
        cloned.copy_from_slice(&self.0);
        Path(cloned)
    }
}

// to make `Metadata` Debug
impl<S> fmt::Debug for Path<S>
where
    S: driver::Storage,
    <S as driver::Storage>::PATH_MAX_PLUS_ONE: ArrayLength<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<S> Path<S>
where
    S: driver::Storage,
    <S as driver::Storage>::PATH_MAX_PLUS_ONE: ArrayLength<u8>,
{
    /// Silently truncates to maximum configured path length
    pub fn new<P: AsRef<[u8]> + ?Sized>(p: &P) -> Self {
        let mut padded_path: GenericArray<u8, S::PATH_MAX_PLUS_ONE> = Default::default();
        let name_max = <S as driver::Storage>::PATH_MAX_PLUS_ONE::to_usize();
        let len = cmp::min(name_max - 1, p.as_ref().len());
        padded_path[..len].copy_from_slice(&p.as_ref()[..len]);
        Path(padded_path)
    }

    pub fn is_absolute(&self) -> bool {
        self.has_root()
    }

    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    pub fn has_root(&self) -> bool {
        self.0.len() > 0 && self.0[0] == b'/'
    }
}

impl<S> From<&str> for Path<S>
where
    S: driver::Storage,
    <S as driver::Storage>::PATH_MAX_PLUS_ONE: ArrayLength<u8>,
{
    fn from(p: &str) -> Path<S> {
        Path::new(p.as_bytes())
    }
}

impl<S> From<&[u8]> for Path<S>
where
    S: driver::Storage,
    <S as driver::Storage>::PATH_MAX_PLUS_ONE: ArrayLength<u8>,
{
    fn from(p: &[u8]) -> Path<S> {
        Path::new(p)
    }
}
