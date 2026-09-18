# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0]

### ⚠️ BREAKING CHANGES

- **Dependency `wp-model-core` 0.9 → 0.10**: upstream renamed the integer type — `Value::Digit` → `Value::Int`, `DataType::Digit` → `DataType::Int`, serde name `"digit"` → `"int"`, `from_digit` → `from_int`; `DataType::Array`'s payload is now `ArraySubtype`. Version bumped to 0.3.0.
  依赖 `wp-model-core` 0.9 → 0.10（上游把整数类型正名：`Value::Digit` → `Value::Int`、`DataType::Digit` → `DataType::Int`、serde 名 `"digit"` → `"int"`、公开构造器 `from_digit` → `from_int`；`DataType::Array` 的载荷改为 `ArraySubtype`），版本升至 0.3.0。

### Dependencies

- `wp-model-core`: `0.9` → `0.10`.
  `wp-model-core`：`0.9` → `0.10`。

## [0.2.0]

### ⚠️ BREAKING CHANGES

- **Dependency `wp-model-core` 0.8 → 0.9**: upstream added `Value::BigUint` / `DataType::BigInt` variants; version bumped to 0.2.0.  
  依赖 `wp-model-core` 0.8 → 0.9（上游新增 `Value::BigUint` / `DataType::BigInt` 变体），版本升至 0.2.0。

### Dependencies

- `wp-model-core`: `0.8` → `0.9`.  
  `wp-model-core`：`0.8` → `0.9`。

[Unreleased]: https://github.com/wp-labs/wp-source-types/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/wp-labs/wp-source-types/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/wp-labs/wp-source-types/compare/v0.1.1...v0.2.0
