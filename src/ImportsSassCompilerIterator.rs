// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An iterator over Sass_Import_Entry imports.
pub struct ImportsSassCompilerIterator
{
	length: usize,
	sass_compiler: *mut Sass_Compiler,
	next_index: usize,
}

impl Iterator for ImportsSassCompilerIterator
{
	type Item = Sass_Import_Entry;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.next_index == self.length
		{
			None
		}
		else
		{
			let next = self.sass_compiler.get_import(self.next_index);
			self.next_index += 1;
			Some(next)
		}
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		(self.length, Some(self.length))
	}
}

impl ExactSizeIterator for ImportsSassCompilerIterator
{
	fn len(&self) -> usize
	{
		self.length
	}
}

impl ImportsSassCompilerIterator
{
	#[inline(always)]
	fn new(sass_compiler: *mut Sass_Compiler) -> Self
	{
		Self
		{
			length: sass_compiler.get_import_length(),
			sass_compiler,
			next_index: 0,
		}
	}
	
	/// Direct access to last import.
	/// Returns None if there are no imports.
	/// If Some, then value is not a null pointer.
	#[inline(always)]
	pub fn last_import(&self) -> Option<Sass_Import_Entry>
	{
		let pointer = self.sass_compiler.get_import_last();
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(pointer)
		}
	}
}
