use crate::cli::commands::{test::TestArgs, CommandExecution};
use assert_matches::assert_matches;
use cairo_rs::serde::deserialize_program::deserialize_program_json;
use serde::de::Expected;
use std::path::PathBuf;

use super::{
	compile_and_list_entrypoints, setup_hint_processor, test_single_entrypoint, TestCommandError,
};

use crate::cli::commands::test::cache::cache::{
	get_cache_path, read_cache_file, Cache, CacheError,
};

pub fn run_single_test(
	test_name: &str,
	test_path: &PathBuf,
) -> Result<(String, bool), TestCommandError> {
	let (_, path_to_compiled, _) = compile_and_list_entrypoints(test_path.to_owned())?;

	let program_json = deserialize_program_json(&path_to_compiled)?;

	test_single_entrypoint(
		program_json,
		test_name.to_string(),
		&setup_hint_processor(),
		None,
	)
}

#[test]
fn test_cairo_contracts() {
	TestArgs {
		root: PathBuf::from("./test_cairo_contracts"),
	}
	.exec()
	.unwrap();
}

#[test]
fn test_cairo_hints() {
	TestArgs {
		root: PathBuf::from("./test_cairo_hints"),
	}
	.exec()
	.unwrap();
}

#[test]
fn read_cache_with_valid_input() {
	let current_dir = std::env::current_dir().unwrap();
	let path_to_cache =
		PathBuf::from(current_dir.join("test_cache_files").join("test_valid_program.json"));
	let cache = read_cache_file(&path_to_cache).unwrap();

	let expected = Cache {
		contract_path: PathBuf::from("test_cairo_contracts/test_valid_program.cairo"),
		compiled_contract_path: PathBuf::from("test_compiled_contracts/test_valid_program.json"),
		hash: "0x0000000000000000000000000000000000000000000000000000000000000001".to_string(),
	};

	assert_eq!(cache, expected);
}

#[test]
fn read_non_existing_cache_file() {
	let current_dir = std::env::current_dir().unwrap();
	let path_to_cache =
		PathBuf::from(current_dir.join("test_cache_files").join("non_existing_cache.json"));
	let result = read_cache_file(&path_to_cache);
	assert_matches!(result, Err(CacheError::FileNotFoundError(_)));
}

#[test]
fn read_existing_cache_with_incorrect_field() {
	let current_dir = std::env::current_dir().unwrap();
	let path_to_cache =
		PathBuf::from(current_dir.join("test_cache_files").join("test_invalid_structure.json"));
	let result = read_cache_file(&path_to_cache);
	assert_matches!(result, Err(CacheError::DeserializeError(_)));
}

#[test]
fn get_cache_path_for_valid_contract_path() {
	// in test_cairo_contracts dir
	let current_dir = std::env::current_dir().unwrap();
	let path_to_contract =
		PathBuf::from(current_dir.join("test_cairo_contracts").join("test_valid_program.cairo"));
	let path_to_cache = get_cache_path(&path_to_contract).unwrap();

	let cache_dir = dirs::cache_dir().unwrap();
	let expected =
		PathBuf::from(cache_dir.join("cairo-foundry-cache").join("test_valid_program.json"));
	assert_eq!(path_to_cache, expected);

	// in project root dir
	let path_to_contract = PathBuf::from("test_valid_program.cairo");
	let path_to_cache = get_cache_path(&path_to_contract).unwrap();

	let cache_dir = dirs::cache_dir().unwrap();
	let expected =
		PathBuf::from(cache_dir.join("cairo-foundry-cache").join("test_valid_program.json"));
	assert_eq!(path_to_cache, expected);
}

#[test]
fn get_cache_path_for_invalid_contract_extension() {
	// incorrect extension
	let path_to_contract = PathBuf::from("test_invalid_extension.sol");
	let path_to_cache = get_cache_path(&path_to_contract);
	assert_matches!(path_to_cache, Err(CacheError::InvalidContractExtension(_)));

	let path_to_contract = PathBuf::from("test_invalid_extension.rs");
	let path_to_cache = get_cache_path(&path_to_contract);
	assert_matches!(path_to_cache, Err(CacheError::InvalidContractExtension(_)));

	// no extension
	let path_to_contract = PathBuf::from("test_invalid_extension");
	let path_to_cache = get_cache_path(&path_to_contract);
	assert_matches!(path_to_cache, Err(CacheError::InvalidContractExtension(_)));
}
