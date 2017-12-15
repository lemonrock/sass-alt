// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An iterator over MapSassValue keys.
pub struct KeyMapSassValueIterator<'a>
{
	length: usize,
	map_sass_value: MapSassValue<'a>,
	next_index: usize,
}

impl<'a> Iterator for KeyMapSassValueIterator<'a>
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
			let next_key = unsafe { self.map_sass_value.get_key_unchecked(self.next_index) };
			self.next_index += 1;
			Some(next_key)
		}
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		(self.length, Some(self.length))
	}
}

impl<'a> ExactSizeIterator for KeyMapSassValueIterator<'a>
{
	fn len(&self) -> usize
	{
		self.length
	}
}

impl<'a> KeyMapSassValueIterator<'a>
{
	#[inline(always)]
	fn new(map_sass_value: MapSassValue<'a>) -> Self
	{
		Self
		{
			length: map_sass_value.length(),
			map_sass_value,
			next_index: 0,
		}
	}
}
