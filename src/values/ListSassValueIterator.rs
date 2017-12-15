// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An iterator over ListSassValue values.
pub struct ListSassValueIterator<'a>
{
	length: usize,
	list_sass_value: ListSassValue<'a>,
	next_index: usize,
}

impl<'a> Iterator for ListSassValueIterator<'a>
{
	type Item = SassValue;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.next_index == self.length
		{
			None
		}
		else
		{
			let next = unsafe { self.list_sass_value.get_value_unchecked(self.next_index) };
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

impl<'a> ExactSizeIterator for ListSassValueIterator<'a>
{
	fn len(&self) -> usize
	{
		self.length
	}
}

impl<'a> ListSassValueIterator<'a>
{
	#[inline(always)]
	fn new(list_sass_value: ListSassValue<'a>) -> Self
	{
		Self
		{
			length: list_sass_value.length(),
			list_sass_value,
			next_index: 0,
		}
	}
}
