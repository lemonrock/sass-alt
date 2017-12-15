// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Wrapper around the Sass_Compiler object to make it easier to use correctly.
#[derive(Debug, Copy, Clone)]
pub struct SassCompiler(*mut Sass_Compiler);

impl SassCompiler
{
	/// Get Sass_Context object.
	#[inline(always)]
	pub fn context(&self) -> *mut Sass_Context
	{
		self.0.get_context()
	}
	
	/// Get Sass_Options object.
	#[inline(always)]
	pub fn get_options(&self) -> *mut Sass_Options
	{
		self.0.get_options()
	}
	
	/// Get compiler state object.
	#[inline(always)]
	pub fn get_state(&self) -> Sass_Compiler_State
	{
		self.0.get_state()
	}
	
	/// Iterate over imports.
	/// Iterator also provides a specific `last_import()` method for convenience.
	#[inline(always)]
	pub fn iter_imports(&self) -> ImportsSassCompilerIterator
	{
		ImportsSassCompilerIterator::new(self.0)
	}
	
	/// Iterate over callees.
	/// Iterator also provides a specific `last_callee()` method for convenience.
	#[inline(always)]
	pub fn iter_callees(&self) -> CalleesSassCompilerIterator
	{
		CalleesSassCompilerIterator::new(self.0)
	}
}
