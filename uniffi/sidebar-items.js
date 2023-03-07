window.SIDEBAR_ITEMS = {"attr":[["export",""]],"constant":[["IDX_CALLBACK_FREE","The method index used by the Drop trait to communicate to the foreign language side that Rust has finished with it, and it can be deleted from the handle map."]],"derive":[["Enum",""],["Error",""],["Object",""],["Record",""]],"fn":[["call_with_output","Wrap a rust function call and return the result directly"],["call_with_result","Wrap a rust function call that returns a `Result<_, RustBuffer>`"],["check_compatible_version","Check whether the uniffi runtime version is compatible a given uniffi_bindgen version."],["check_remaining","A helper function to ensure we don’t read past the end of a buffer."],["generate_bindings",""],["generate_component_scaffolding",""],["generate_scaffolding","Generate the rust “scaffolding” required to build a uniffi component."],["lower_anyhow_error_or_panic",""],["print_json",""],["uniffi_bindgen_main",""],["uniffi_rustbuffer_alloc","This helper allocates a new byte buffer owned by the Rust code, and returns it to the foreign-language code as a `RustBuffer` struct. Callers must eventually free the resulting buffer, either by explicitly calling [`uniffi_rustbuffer_free`] defined below, or by passing ownership of the buffer back into Rust code."],["uniffi_rustbuffer_free","Free a byte buffer that had previously been passed to the foreign language code."],["uniffi_rustbuffer_from_bytes","This helper copies bytes owned by the foreign-language code into a new byte buffer owned by the Rust code, and returns it as a `RustBuffer` struct. Callers must eventually free the resulting buffer, either by explicitly calling the destructor defined below, or by passing ownership of the buffer back into Rust code."],["uniffi_rustbuffer_reserve","Reserve additional capacity in a byte buffer that had previously been passed to the foreign language code."]],"macro":[["assert_compatible_version","Assert that the uniffi runtime version matches an expected value."],["include_scaffolding","A helper macro to include generated component scaffolding."]],"mod":[["deps",""],["ffi",""]],"struct":[["ForeignBytes","Support for reading a slice of foreign-language-allocated bytes over the FFI."],["ForeignCallbackInternals","Struct to hold a foreign callback."],["RustBuffer","Support for passing an allocated-by-Rust buffer of bytes over the FFI."],["RustCallStatus","Represents the success/error of a rust call"],["UnexpectedUniFFICallbackError","Used when internal/unexpected error happened when calling a foreign callback, for example when a unknown exception is raised"]],"trait":[["FfiConverter","Trait defining how to transfer values via the FFI layer."],["FfiError",""],["RustBufferFfiConverter","A helper trait to implement lowering/lifting using a `RustBuffer`"]],"type":[["ForeignCallback","ForeignCallback is the Rust representation of a foreign language function. It is the basis for all callbacks interfaces. It is registered exactly once per callback interface, at library start up time. Calling this method is only done by generated objects which mirror callback interfaces objects in the foreign language."],["Result","`Result<T, Error>`"]]};