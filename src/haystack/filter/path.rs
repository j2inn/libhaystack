// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack filter path

use crate::haystack::encoding::zinc::decode::id::Id;
use std::fmt::{Display, Formatter, Result};
use std::ops::Index;

/// A `Filter` `Path` element
///
/// Used to resolve indirections such as `siteRef->dis`.
///
/// Path elements are name of [Ref](crate::val::Ref) tags in a Dict that would
/// be resolved to other Dicts that have the same `id` tag as the value of the Ref path element.
/// This indirection scheme allows chaining multiple dicts in a graph like arrangement.
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Path {
    segments: Vec<Id>,
}

impl Path {
    /// `True` if this `Path` is empty
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// `Path`'s length
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Obtain an `Id` iterator over `Path`s segments
    pub fn iter(&self) -> std::slice::Iter<Id> {
        self.segments.iter()
    }
}

/// Creates `Path` from `Vec<Id>`
impl From<Vec<Id>> for Path {
    fn from(segments: Vec<Id>) -> Self {
        Path { segments }
    }
}

/// Creates `Path` from [Id](haystack::decoding::zinc::Id)
impl From<Id> for Path {
    fn from(segment: Id) -> Self {
        Path {
            segments: vec![segment],
        }
    }
}

/// Creates `Path` from `&str`
impl From<&str> for Path {
    fn from(segment: &str) -> Self {
        Path::from(Id::from(segment))
    }
}

impl Index<usize> for Path {
    type Output = Id;

    fn index(&self, index: usize) -> &Self::Output {
        &self.segments[index]
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.segments
            .iter()
            .enumerate()
            .try_for_each(|(idx, id)| -> Result {
                f.write_str(&id.to_string())?;
                if idx < self.segments.len() - 1 {
                    f.write_str("->")
                } else {
                    Ok(())
                }
            })
    }
}
