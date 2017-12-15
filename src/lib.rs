// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of sass-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


#![deny(missing_docs)]


//! This crate provides a simple API that allows one to compile SASS and use Rust functions as SASS functions, importers and headers.
//! It is used by the Cordial crate to allow Lua code to be embedded inside SASS.
//! To get started:-
//!
//! ```
//! extern crate sass_alt;
//!
//! use sass_alt::FunctionList;
//! use sass_alt::InputSyntax;
//! use sass_alt::SassOptions;
//!
//! fn main()
//! {
//! 	let sass_functions = vec![];
//! 	let function_list = Rc::new(SassFunctionList(sass_functions));
//!
//! 	let importer_functions = vec![];
//! 	let importer_list = Rc::new(SassImportersList(importer_functions));
//!
//! 	let header_functions = vec![];
//! 	let header_list = Rc::new(SassImportersList(header_functions));
//!
//! 	let css = SassOptions::minified_css(InputSyntax::SCSS, &["/path/to/sass/includes"], &function_list, &importer_list, &header_list).compile_data(".hello { color: red }").unwrap();
//! }
//! ```
//!


extern crate libc;
#[macro_use] extern crate quick_error;
pub extern crate sass_sys;


use self::values::*;
use ::libc::c_char;
use ::libc::c_void;
use ::sass_sys::*;
use ::std::borrow::Cow;
use ::std::clone::Clone;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::OsString;
use ::std::fmt::Debug;
use ::std::mem::forget;
use ::std::path::Path;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::std::rc::Rc;
use ::std::str::Utf8Error;


/// Wrappers for Sass_Value
pub mod values;


include!("CalleesSassCompilerIterator.rs");
include!("DataOrFileSassContextMutPointer.rs");
include!("ImportsSassCompilerIterator.rs");
include!("InputSyntax.rs");
include!("OutputStyle.rs");
include!("Sass_Callee_Entry_Ext.rs");
include!("Sass_Compiler_Ext.rs");
include!("Sass_Context_Ext.rs");
include!("Sass_Env_Frame_Ext.rs");
include!("Sass_Function_Entry_Ext.rs");
include!("Sass_Function_List_Ext.rs");
include!("Sass_Import_Entry_Ext.rs");
include!("Sass_Import_List_Ext.rs");
include!("Sass_Importer_Entry_Ext.rs");
include!("Sass_Importer_List_Ext.rs");
include!("Sass_Options_Ext.rs");
include!("SassCompileError.rs");
include!("SassCompiler.rs");
include!("SassContext.rs");
include!("SassFunction.rs");
include!("SassFunctionList.rs");
include!("SassFunctionSignature.rs");
include!("SassFunctionTraitObject.rs");
include!("SassImport.rs");
include!("SassImportEntry.rs");
include!("SassImporter.rs");
include!("SassImporterError.rs");
include!("SassImporterList.rs");
include!("SassImporterTraitObject.rs");
include!("SassOptions.rs");
