// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


quick_error!
{
	/// A sass value conversion or usage error.
	#[derive(Debug)]
	pub enum SassValueError
	{
		/// SASS function failed in some way.
		FunctionFailed(reason: Cow<'static, str>)
		{
			description(reason.as_ref())
			display("Function failed")
		}
		
		/// Could not convert C string to UTF-8.
		Utf8ConversionFromCStringFailed(cause: Utf8Error)
		{
			description(cause.description())
			display("Could not convert to UTF-8 from C String because {}", cause)
			from()
		}
	
		/// Is not a boolean.
		IsNotABoolean
		{
			description("Is not a boolean")
			display("Is not a boolean")
		}
		
		/// Is not a boolean or null.
		IsNotABooleanOrNull
		{
			description("Is not a boolean or null")
			display("Is not a boolean or null")
		}
		
		/// Is not a color.
		IsNotAColor
		{
			description("Is not a color")
			display("Is not a color")
		}
		
		/// Is not a color or null.
		IsNotAColorOrNull
		{
			description("Is not a color or null")
			display("Is not a color or null")
		}
		
		/// Is not an error.
		IsNotAnError
		{
			description("Is not an error")
			display("Is not an error")
		}
		
		/// Is not an error or null.
		IsNotAnErrorOrNull
		{
			description("Is not an error or null")
			display("Is not an error or null")
		}
		
		/// Is not a list.
		IsNotAList
		{
			description("Is not a list")
			display("Is not a list")
		}
		
		/// Is not a list or null.
		IsNotAListOrNull
		{
			description("Is not a list or null")
			display("Is not a list or null")
		}
		
		/// Is not a map.
		IsNotAMap
		{
			description("Is not a map")
			display("Is not a map")
		}
		
		/// Is not a map or null.
		IsNotAMapOrNull
		{
			description("Is not a map or null")
			display("Is not a map or null")
		}
		
		/// Is not a number.
		IsNotANumber
		{
			description("Is not a number")
			display("Is not a number")
		}
		
		/// Is not a number or null.
		IsNotANumberOrNull
		{
			description("Is not a number or null")
			display("Is not a number or null")
		}
		
		/// Is not a string.
		IsNotAString
		{
			description("Is not a string")
			display("Is not a string")
		}
		
		/// Is not a string or null.
		IsNotAStringOrNull
		{
			description("Is not a string or null")
			display("Is not a string or null")
		}
		
		/// Is not a warning.
		IsNotAWarning
		{
			description("Is not a warning")
			display("Is not a warning")
		}
		
		/// Is not a warning or null.
		IsNotAWarningOrNull
		{
			description("Is not a warning or null")
			display("Is not a warning or null")
		}
	}
}

impl SassValueError
{
	/// Convenience method for creation FunctionFailed error.
	#[inline(always)]
	pub fn function_failed_from_static_str(reason: &'static str) -> Self
	{
		SassValueError::FunctionFailed(Cow::Borrowed(reason))
	}
	
	/// Convenience method for creation FunctionFailed error.
	#[inline(always)]
	pub fn function_failed_from_string(reason: String) -> Self
	{
		SassValueError::FunctionFailed(Cow::Owned(reason))
	}
}
