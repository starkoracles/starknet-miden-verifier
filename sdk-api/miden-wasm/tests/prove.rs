use hex::ToHex;
use log::info;
use miden_wasm::{
    convert::sdk::sdk::{self, FieldExtension, HashFunction, MidenPublicInputs, PrimeField},
    MidenProver,
};
use prost::Message;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn prove_fib() {
    log::set_logger(&DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let program_10_fib = "
    begin 
        repeat.10
            swap dup.1 add
        end
    end";

    let program = sdk::MidenProgram {
        program: program_10_fib.to_string(),
        ..Default::default()
    };

    let program_inputs = sdk::MidenProgramInputs {
        stack_init: vec![0, 1],
        advice_tape: vec![],
        ..Default::default()
    };

    let proof_options = sdk::ProofOptions {
        num_queries: 27,
        blowup_factor: 8,
        grinding_factor: 16,
        hash_fn: HashFunction::Blake2s.into(),
        field_extension: FieldExtension::None.into(),
        fri_folding_factor: 8,
        fri_max_remainder_size: 256,
        prime_field: PrimeField::Goldilocks.into(),
        ..Default::default()
    };

    let mut miden_prover = MidenProver::new();

    let prover_output = miden_prover
        .prove(
            program.encode_to_vec(),
            program_inputs.encode_to_vec(),
            proof_options.encode_to_vec(),
        )
        .unwrap();

    let sdk_output = sdk::MidenProgramOutputs::decode(&prover_output.program_outputs[..]).unwrap();
    let pub_inputs: MidenPublicInputs =
        sdk::MidenPublicInputs::decode(&prover_output.public_inputs[..])
            .unwrap()
            .into();

    let u64_stack = sdk_output
        .stack
        .iter()
        .map(|field_element| field_element.into())
        .collect::<Vec<u64>>();

    info!("outputs: {:?}", u64_stack);
    info!(
        "Program hash: {:?}",
        &pub_inputs.program_hash.unwrap().data.encode_hex::<String>()
    );
}
