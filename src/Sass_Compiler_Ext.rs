// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Compiler class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
pub trait Sass_Compiler_Ext
{
	/// Get Sass_Context object.
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context;
	
	/// Get Sass_Options object.
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options;
	
	/// Get compiler state object.
	#[inline(always)]
	fn get_state(self) -> Sass_Compiler_State;
	
	/// Get number of imports.
	#[inline(always)]
	fn get_import_length(self) -> usize;
	
	/// Get Sass_Import_Entry object (can be null if equal to or greater than `get_import_length()`).
	#[inline(always)]
	fn get_import(self, index: usize) -> Sass_Import_Entry;
	
	/// Get Sass_Import_Entry object (can be null if `get_import_length()` is zero).
	#[inline(always)]
	fn get_import_last(self) -> Sass_Import_Entry;
	
	/// Get number of callees.
	#[inline(always)]
	fn get_callee_length(self) -> usize;
	
	/// Get Sass_Import_Entry object (can be null if equal to or greater than `get_callee_length()`).
	#[inline(always)]
	fn get_callee(self, index: usize) -> Sass_Callee_Entry;
	
	/// Get Sass_Import_Entry object (can be null if `get_callee_length()` is zero).
	#[inline(always)]
	fn get_callee_last(self) -> Sass_Callee_Entry;
}

impl Sass_Compiler_Ext for *mut Sass_Compiler
{
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context
	{
		unsafe { sass_compiler_get_context(self) }
	}
	
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options
	{
		unsafe { sass_compiler_get_options(self) }
	}
	
	/// Compiler state object.
	#[inline(always)]
	fn get_state(self) -> Sass_Compiler_State
	{
		unsafe { sass_compiler_get_state(self) }
	}
	
	#[inline(always)]
	fn get_import_length(self) -> usize
	{
		unsafe { sass_compiler_get_import_stack_size(self) }
	}
	
	#[inline(always)]
	fn get_import(self, index: usize) -> Sass_Import_Entry
	{
		unsafe { sass_compiler_get_import_entry(self, index) }
	}
	
	#[inline(always)]
	fn get_import_last(self) -> Sass_Import_Entry
	{
		unsafe { sass_compiler_get_last_import(self) }
	}
	
	#[inline(always)]
	fn get_callee_length(self) -> usize
	{
		unsafe { sass_compiler_get_callee_stack_size(self) }
	}
	
	#[inline(always)]
	fn get_callee(self, index: usize) -> Sass_Callee_Entry
	{
		unsafe { sass_compiler_get_callee_entry(self, index) }
	}
	
	#[inline(always)]
	fn get_callee_last(self) -> Sass_Callee_Entry
	{
		unsafe { sass_compiler_get_last_callee(self) }
	}
}
