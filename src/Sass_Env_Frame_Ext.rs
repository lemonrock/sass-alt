// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the Sass_Env_Frame_Ext class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
pub trait Sass_Env_Frame_Ext
{
	/// Get a lexical environment variable. We own the value returned, as it is copy of that inside libsass.
	#[inline(always)]
	fn get_lexical_environment_variable(self, name: &CStr) -> SassValue;
	
	/// Set a lexical environment variable. A copy is made and altered inside libsass.
	#[inline(always)]
	fn set_lexical_environment_variable(self, name: &CStr, value: &SassValue);
	
	/// Get a local environment variable. We own the value returned, as it is copy of that inside libsass.
	#[inline(always)]
	fn get_local_environment_variable(self, name: &CStr) -> SassValue;
	
	/// Set a local environment variable. A copy is made and altered inside libsass.
	#[inline(always)]
	fn set_local_environment_variable(self, name: &CStr, value: &SassValue);
	
	/// Get a global environment variable. We own the value returned, as it is copy of that inside libsass.
	#[inline(always)]
	fn get_global_environment_variable(self, name: &CStr) -> SassValue;
	
	/// Set a global environment variable. A copy is made and altered inside libsass.
	#[inline(always)]
	fn set_global_environment_variable(self, name: &CStr, value: &SassValue);
}

impl Sass_Env_Frame_Ext for Sass_Env_Frame
{
	#[inline(always)]
	fn get_lexical_environment_variable(self, name: &CStr) -> SassValue
	{
		SassValue(unsafe { sass_env_get_lexical(self, name.as_ptr()) }, true)
	}
	
	#[inline(always)]
	fn set_lexical_environment_variable(self, name: &CStr, value: &SassValue)
	{
		unsafe { sass_env_set_lexical(self, name.as_ptr(), value.0) }
	}
	
	#[inline(always)]
	fn get_local_environment_variable(self, name: &CStr) -> SassValue
	{
		SassValue(unsafe { sass_env_get_local(self, name.as_ptr()) }, true)
	}
	
	#[inline(always)]
	fn set_local_environment_variable(self, name: &CStr, value: &SassValue)
	{
		unsafe { sass_env_set_local(self, name.as_ptr(), value.0) }
	}
	
	#[inline(always)]
	fn get_global_environment_variable(self, name: &CStr) -> SassValue
	{
		SassValue(unsafe { sass_env_get_global(self, name.as_ptr()) }, true)
	}
	
	#[inline(always)]
	fn set_global_environment_variable(self, name: &CStr, value: &SassValue)
	{
		unsafe { sass_env_set_global(self, name.as_ptr(), value.0) }
	}
}
