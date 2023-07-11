// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{
    FRI_MAX_FOLDING_FACTOR, FRI_MAX_REMAINDER_DEGREE, FRI_MIN_FOLDING_FACTOR, MAX_BLOWUP_FACTOR,
    MAX_GRINDING_FACTOR, MAX_NUM_QUERIES, MIN_BLOWUP_FACTOR,
};
use core::fmt;

// ASSERTION ERROR
// ================================================================================================
/// Represents an error returned during assertion evaluation.
#[derive(Debug, PartialEq, Eq)]
pub enum AssertionError {
    /// This error occurs when an assertion is evaluated against an execution trace which does not
    /// contain a column specified by the assertion.
    TraceWidthTooShort(usize, usize),
    /// This error occurs when an assertion is evaluated against an execution trace with length
    /// which is not a power of two.
    TraceLengthNotPowerOfTwo(usize),
    /// This error occurs when an assertion is evaluated against an execution trace which does not
    /// contain a step against which the assertion is placed.
    TraceLengthTooShort(usize, usize),
    /// This error occurs when a `Sequence` assertion is placed against an execution trace with
    /// length which conflicts with the trace length implied by the assertion.
    TraceLengthNotExact(usize, usize),
}

impl fmt::Display for AssertionError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TraceWidthTooShort(expected, actual) => {
                write!(f, "expected trace width to be at least {expected}, but was {actual}")
            }
            Self::TraceLengthNotPowerOfTwo(actual) => {
                write!(f, "expected trace length to be a power of two, but was {actual}")
            }
            Self::TraceLengthTooShort(expected, actual) => {
                write!(f, "expected trace length to be at least {expected}, but was {actual}")
            }
            Self::TraceLengthNotExact(expected, actual) => {
                write!(f, "expected trace length to be exactly {expected}, but was {actual}")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProofError {
    QueriesNumber(usize),
    BlowupFactor(usize),
    GrindingFactor(u32),
    FoldingFactor(usize),
    FriRemainder(usize),
}

impl fmt::Display for ProofError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO: maybe move constants to this file
            Self::QueriesNumber(num_queries) => {
                write!(f, "Number of queries must be greater than 0 and smaller than {MAX_NUM_QUERIES}, but {num_queries} was found.")
            }
            Self::BlowupFactor(blowup_factor) => {
                write!(f, "Blowup factor must be a power of 2, cannot be smaller than {MIN_BLOWUP_FACTOR} and greater than {MAX_BLOWUP_FACTOR}, but {blowup_factor} was found.")
            }
            Self::GrindingFactor(grinding_factor) => {
                write!(f, "Grinding factor cannot be greater than {MAX_GRINDING_FACTOR}, but {grinding_factor} was found.")
            }
            Self::FoldingFactor(fri_folding_factor) => {
                write!(f, "FRI folding factor must be a power of 2, cannot be smaller than {FRI_MIN_FOLDING_FACTOR} and greater than {FRI_MAX_FOLDING_FACTOR}, but {fri_folding_factor} was found.")
            }
            Self::FriRemainder(fri_remainder_max_degree) => {
                write!(f, "FRI polynomial remainder degree must be one less than a power of two and cannot be greater than {FRI_MAX_REMAINDER_DEGREE}, but {fri_remainder_max_degree} was found.")
            }
        }
    }
}
