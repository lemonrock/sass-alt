// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Sass_Value typed as a known string.
#[derive(Debug, Copy, Clone)]
pub struct ListSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> ListSassValue<'a>
{
	/// Get length.
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		unsafe { sass_list_get_length(self.reference.0 as *const _) }
	}
	
	/// Get separator.
	#[inline(always)]
	pub fn separator(&self) -> ListSeparator
	{
		unsafe { sass_list_get_separator(self.reference.0 as *const _) }.into()
	}
	
	/// Set separator.
	#[inline(always)]
	pub fn set_separator(&self, separator: ListSeparator)
	{
		unsafe { sass_list_set_separator(self.reference.0, separator.into()) }
	}
	
	/// Get whether this is a bracketed list.
	#[inline(always)]
	pub fn is_bracketed(&self) -> bool
	{
		false
		//unsafe { sass_list_get_is_bracketed(self.reference.0 as *const _) }
	}
	
	/// Set whether this is a bracketed list.
	#[inline(always)]
	pub fn set_is_bracketed(&self, _is_bracketed: bool)
	{
		//unsafe { sass_list_set_is_bracketed(self.reference.0, _is_bracketed) }
	}
	
	/// Get value.
	/// There is no bounds check.
	#[inline(always)]
	pub unsafe fn get_value_unchecked(&self, index: usize) -> SassValue
	{
		let pointer = sass_list_get_value(self.reference.0 as *const _, index);
		SassValue(pointer, self.reference.is_owned_by_rust())
	}
	
	/// Set value.
	/// There is no bounds check.
	#[inline(always)]
	pub unsafe fn set_value_unchecked(&self, index: usize, value: SassValue)
	{
		sass_list_set_value(self.reference.0, index, value.transfer_ownership_to_c());
	}
	
	/// Iterator over values
	#[inline(always)]
	pub fn iter(&self) -> ListSassValueIterator<'a>
	{
		ListSassValueIterator::new(*self)
	}
}

impl<'a> IntoIterator for ListSassValue<'a>
{
	type Item = SassValue;
	
	type IntoIter = ListSassValueIterator<'a>;
	
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		ListSassValueIterator::new(self)
	}
}
