// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Data_Context and *mut Sass_File_Context class to make their methods Object-Orientated and interchangeable.
#[allow(non_camel_case_types)]
pub trait DataOrFileSassContextMutPointer: Copy
{
	/// Delete (drop) this object. Guards that this object is not null.
	#[inline(always)]
	fn delete(self);
	
	/// Compile using this context. Error code seems to be returned but libsass documentation is not clear on this.
	#[inline(always)]
	fn compile(self) -> i32;
	
	/// Get Sass_Context object.
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context;
	
	/// Get Sass_Options object.
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options;
}

impl DataOrFileSassContextMutPointer for *mut Sass_Data_Context
{
	#[inline(always)]
	fn delete(self)
	{
		if !self.is_null()
		{
			unsafe { sass_delete_data_context(self) };
		}
	}
	
	#[inline(always)]
	fn compile(self) -> i32
	{
		unsafe { sass_compile_data_context(self) }
	}
	
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context
	{
		unsafe { sass_data_context_get_context(self) }
	}
	
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options
	{
		unsafe { sass_data_context_get_options(self) }
	}
}

impl DataOrFileSassContextMutPointer for *mut Sass_File_Context
{
	#[inline(always)]
	fn delete(self)
	{
		if !self.is_null()
		{
			unsafe { sass_delete_file_context(self) };
		}
	}
	
	#[inline(always)]
	fn compile(self) -> i32
	{
		unsafe { sass_compile_file_context(self) }
	}
	
	#[inline(always)]
	fn get_context(self) -> *mut Sass_Context
	{
		unsafe { sass_file_context_get_context(self) }
	}
	
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options
	{
		unsafe { sass_file_context_get_options(self) }
	}
}
