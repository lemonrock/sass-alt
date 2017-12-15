// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Sass_Value typed as a known map.
/// Note that these are not real maps. They are actually just vectors of (Key, Value) pairs, and the Key can be duplicated.
#[derive(Debug, Copy, Clone)]
pub struct MapSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> MapSassValue<'a>
{
	/// Is empty.
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		self.length() == 0
	}
	
	/// Get length.
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		unsafe { sass_map_get_length(self.reference.0 as *const _) }
	}
	
	/// Get key.
	/// There is no bounds check.
	#[inline(always)]
	pub unsafe fn get_key_unchecked(&self, index: usize) -> SassValue
	{
		let pointer = sass_map_get_key(self.reference.0 as *const _, index);
		SassValue(pointer, self.reference.is_owned_by_rust())
	}
	
	/// Set key.
	/// There is no bounds check.
	#[inline(always)]
	pub unsafe fn set_key_unchecked(&self, index: usize, key: SassValue)
	{
		sass_map_set_key(self.reference.0, index, key.transfer_ownership_to_c())
	}
	
	/// Get value.
	/// There is no bounds check.
	#[inline(always)]
	pub unsafe fn get_value_unchecked(&self, index: usize) -> SassValue
	{
		let pointer = sass_map_get_value(self.reference.0 as *const _, index);
		SassValue(pointer, self.reference.is_owned_by_rust())
	}
	
	/// Set value.
	/// There is no bounds check.
	#[inline(always)]
	pub unsafe fn set_value_unchecked(&self, index: usize, value: SassValue)
	{
		sass_map_set_value(self.reference.0, index, value.transfer_ownership_to_c())
	}
	
	/// Iterator over (key, value) pairs.
	#[inline(always)]
	pub fn iter(&self) -> MapSassValueIterator<'a>
	{
		MapSassValueIterator::new(*self)
	}
	
	/// Iterator over keys.
	#[inline(always)]
	pub fn iter_keys(&self) -> KeyMapSassValueIterator<'a>
	{
		KeyMapSassValueIterator::new(*self)
	}
	
	/// Iterator over values.
	#[inline(always)]
	pub fn iter_values(&self) -> ValueMapSassValueIterator<'a>
	{
		ValueMapSassValueIterator::new(*self)
	}
}

impl<'a> IntoIterator for MapSassValue<'a>
{
	type Item = (SassValue, SassValue);
	
	type IntoIter = MapSassValueIterator<'a>;
	
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter
	{
		MapSassValueIterator::new(self)
	}
}
