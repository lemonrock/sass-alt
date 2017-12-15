// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A wrapper type that holds a list of SASS functions and their Rust representations so that Rust function lifetimes exceed references to SASS functions held by the libsass C object Sass_Function_List.
#[derive(Debug)]
pub struct SassFunctionList(Sass_Function_List, Vec<Box<SassFunctionTraitObject>>);

impl Drop for SassFunctionList
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.delete()
	}
}

impl Default for SassFunctionList
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(vec![])
	}
}

impl SassFunctionList
{
	/// Create a new list of SASS functions. Called by value as we need to take ownership of the SASS functions to manage their lifetimes.
	#[inline(always)]
	pub fn new(sass_functions: Vec<SassFunctionTraitObject>) -> Self
	{
		let list = Sass_Function_List::make(sass_functions.len());
		let mut drop_sass_functions_when_function_list_drops = Vec::with_capacity(sass_functions.len());
		let mut index = 0;
		for sass_function in sass_functions
		{
			let signature = CString::new(sass_function.signature().as_str()).unwrap();
			
			let mut cookie_holder: Box<SassFunctionTraitObject> = Box::new(sass_function);
			let cookie = cookie_holder.as_mut() as *mut _ as *mut ::std::os::raw::c_void;
			
			let function_entry = unsafe { sass_make_function(signature.as_ptr(), Some(Self::call), cookie) };
			
			drop_sass_functions_when_function_list_drops.push(cookie_holder);
			list.set_list_entry(index, function_entry);
			index += 1;
		}
		SassFunctionList(list, drop_sass_functions_when_function_list_drops)
	}
	
	extern "C" fn call(s_args: *const Sass_Value, cb: Sass_Function_Entry, comp: *mut Sass_Compiler) -> *mut Sass_Value
	{
		let cookie = cb.get_cookie();
		let raw_this = cookie as *mut SassFunctionTraitObject;
		let this = unsafe { &mut *raw_this };
		let arguments = SassValue(s_args as *mut _, false);
		
		let result = if let Ok(Some(arguments_list)) = arguments.as_list()
		{
			match this.callback(arguments_list, SassCompiler(comp))
			{
				Ok(result) => result,
				Err(error) =>
				{
					if let Ok(string) = CString::new(error.as_ref())
					{
						SassValue::new_error(&string)
					}
					else
					{
						SassValue::new_error(&CString::new("error from custom SASS function was invalid").unwrap())
					}
				},
			}
		}
		else
		{
			SassValue::new_error(&CString::new("arguments supplied were not a list; is this a bug in libsass?").unwrap())
		};
		result.transfer_ownership_to_c()
	}
	
	#[inline(always)]
	fn set_functions_on_options(&self, options: *mut Sass_Options)
	{
		if !self.1.is_empty()
		{
			options.set_c_functions(self.0);
		}
	}
}
