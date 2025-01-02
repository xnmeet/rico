//! Abstract Syntax Tree (AST) definitions for Thrift IDL.
//!
//! This module defines the data structures that represent Thrift IDL constructs
//! in memory. The AST is designed to be:
//!
//! - Complete: Represents all Thrift language features
//! - Accurate: Preserves source information and comments
//! - Serializable: Can be converted to/from JSON
//! - Traversable: Easy to analyze and transform
//!
//! # Module Structure
//!
//! The AST module is organized into several submodules:
//!
//! - `definitions`: Core Thrift constructs (structs, services, etc.)
//! - `types`: Type system representations
//! - `values`: Constant and default value representations
//!
//! # AST Node Types
//!
//! ## Document Level
//! - Document: Root node containing all definitions
//! - Namespace: Language-specific namespace declarations
//! - Include: File inclusion statements
//!
//! ## Type Definitions
//! - Struct: Record type definitions
//! - Union: Tagged union definitions
//! - Exception: Error type definitions
//! - Enum: Enumerated type definitions
//! - Typedef: Type alias definitions
//!
//! ## Service Definitions
//! - Service: RPC service interface
//! - Function: Service method definitions
//! - Field: Structure and parameter fields
//!
//! ## Type System
//! - Base types (i32, string, etc.)
//! - Container types (list, set, map)
//! - User-defined type references
//!
//! ## Values
//! - Constants
//! - Default values
//! - Enum values
//!
//! # Source Information
//!
//! Each AST node includes:
//! - Source location (line and column)
//! - Associated comments
//! - Annotations and metadata
//!
//! # Serialization
//!
//! The AST can be serialized to JSON format for:
//! - Debugging and visualization
//! - Integration with other tools
//! - Persistence and caching
//!
//! # Usage
//!
//! The AST is typically created by the parser and can be:
//! - Analyzed for validation
//! - Transformed for optimization
//! - Converted to other formats
//! - Written back to Thrift IDL

mod definitions;
mod types;

pub use self::definitions::*;
pub use self::types::*;
