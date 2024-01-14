// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::utils::compute_mulfib_term;
use crate::{Blake3_192, Blake3_256, Example, ExampleOptions, HashFunction, Sha3_256};
use core::marker::PhantomData;
use std::time::Instant;
use tracing::{debug_span, event, Level};
use winterfell::{
    crypto::{DefaultRandomCoin, ElementHasher},
    math::{fields::f128::BaseElement, FieldElement},
    ProofOptions, Prover, StarkProof, Trace, VerifierError,
};

mod air;
use air::MulFib2Air;

mod prover;
use prover::MulFib2Prover;

#[cfg(test)]
mod tests;

// FIBONACCI EXAMPLE
// ================================================================================================

pub fn get_example(
    options: &ExampleOptions,
    sequence_length: usize,
) -> Result<Box<dyn Example>, String> {
    let (options, hash_fn) = options.to_proof_options(28, 8);

    match hash_fn {
        HashFunction::Blake3_192 => {
            Ok(Box::new(MulFib2Example::<Blake3_192>::new(sequence_length, options)))
        }
        HashFunction::Blake3_256 => {
            Ok(Box::new(MulFib2Example::<Blake3_256>::new(sequence_length, options)))
        }
        HashFunction::Sha3_256 => {
            Ok(Box::new(MulFib2Example::<Sha3_256>::new(sequence_length, options)))
        }
        _ => Err("The specified hash function cannot be used with this example.".to_string()),
    }
}
pub struct MulFib2Example<H: ElementHasher> {
    options: ProofOptions,
    sequence_length: usize,
    result: BaseElement,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> MulFib2Example<H> {
    pub fn new(sequence_length: usize, options: ProofOptions) -> Self {
        assert!(sequence_length.is_power_of_two(), "sequence length must be a power of 2");

        // compute Fibonacci sequence
        let now = Instant::now();
        let result = compute_mulfib_term(sequence_length);
        println!(
            "Computed multiplicative Fibonacci sequence up to {}th term in {} ms",
            sequence_length,
            now.elapsed().as_millis()
        );

        MulFib2Example {
            options,
            sequence_length,
            result,
            _hasher: PhantomData,
        }
    }
}

// EXAMPLE IMPLEMENTATION
// ================================================================================================

impl<H: ElementHasher> Example for MulFib2Example<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    fn prove(&self) -> StarkProof {
        let sequence_length = self.sequence_length;
        event!(
            Level::DEBUG,
            "Generating proof for computing multiplicative Fibonacci sequence (2 terms per step) up to {}th term",
            sequence_length
        );

        // create a prover
        let prover = MulFib2Prover::<H>::new(self.options.clone());

        // generate execution trace
        let trace = debug_span!("Generating execution trace").in_scope(|| {
            let trace = prover.build_trace(sequence_length);
            let trace_width = trace.width();
            let trace_length = trace.length();
            event!(
                Level::TRACE,
                "Generated execution trace of {} registers and 2^{} steps",
                trace_width,
                trace_length.ilog2(),
            );
            trace
        });

        // generate the proof
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        let acceptable_options =
            winterfell::AcceptableOptions::OptionSet(vec![proof.options().clone()]);
        winterfell::verify::<MulFib2Air, H, DefaultRandomCoin<H>>(
            proof,
            self.result,
            &acceptable_options,
        )
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        let acceptable_options =
            winterfell::AcceptableOptions::OptionSet(vec![proof.options().clone()]);
        winterfell::verify::<MulFib2Air, H, DefaultRandomCoin<H>>(
            proof,
            self.result + BaseElement::ONE,
            &acceptable_options,
        )
    }
}
