#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

    #[test]
    fn test_logs_input_string() {
        let program_id_keypair_bytes = std::fs::read("deploy/assembly_one-keypair.json")
            .unwrap()[..32]
            .try_into()
            .expect("slice with incorrect length");
        let program_id = Pubkey::new_from_array(program_id_keypair_bytes);

        let input_data = b"hello solana!";

        let instruction = Instruction::new_with_bytes(program_id, input_data, vec![]);

        let mollusk = Mollusk::new(&program_id, "deploy/assembly_one");

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &[],
            &[Check::success()],
        );

        assert!(
    result.program_result.logs.iter().any(|log| log.contains("hello solana!")),
    "Expected log not found: {:?}",
    result.program_result.logs
);

    }
}
