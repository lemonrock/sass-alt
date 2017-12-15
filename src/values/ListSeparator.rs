// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// Implements a more restricted range than Sass_Separator
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ListSeparator
{
	/// Comma.
	Comma,
	
	/// Space.
	Space,
}

impl From<Sass_Separator> for ListSeparator
{
	#[inline(always)]
	fn from(separator: Sass_Separator) -> Self
	{
		use self::Sass_Separator::*;
		use self::ListSeparator::*;
		
		match separator
		{
			SASS_COMMA => Comma,
			SASS_SPACE => Space,
			SASS_HASH => panic!("Sass_Separator::HASH is supposed to be internal"),
		}
	}
}

impl Into<Sass_Separator> for ListSeparator
{
	#[inline(always)]
	fn into(self) -> Sass_Separator
	{
		use self::ListSeparator::*;
		use self::Sass_Separator::*;
		
		match self
		{
			Comma => SASS_COMMA,
			Space => SASS_SPACE,
		}
	}
}
