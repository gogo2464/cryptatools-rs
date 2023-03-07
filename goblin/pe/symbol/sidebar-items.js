window.SIDEBAR_ITEMS = {"constant":[["COFF_SYMBOL_SIZE","Size of a single symbol in the COFF Symbol Table."],["IMAGE_COMDAT_SELECT_ANY","Any section that defines the same COMDAT symbol can be linked; the rest are removed."],["IMAGE_COMDAT_SELECT_ASSOCIATIVE","The section is linked if a certain other COMDAT section is linked."],["IMAGE_COMDAT_SELECT_EXACT_MATCH","The linker chooses an arbitrary section among the definitions for this symbol."],["IMAGE_COMDAT_SELECT_LARGEST","The linker chooses the largest definition from among all of the definitions for this symbol."],["IMAGE_COMDAT_SELECT_NODUPLICATES","If this symbol is already defined, the linker issues a “multiply defined symbol” error."],["IMAGE_COMDAT_SELECT_SAME_SIZE","The linker chooses an arbitrary section among the definitions for this symbol."],["IMAGE_SYM_ABSOLUTE","The symbol has an absolute (non-relocatable) `value` and is not an address."],["IMAGE_SYM_CLASS_ARGUMENT","A formal argument (parameter) of a function."],["IMAGE_SYM_CLASS_AUTOMATIC","The automatic (stack) variable."],["IMAGE_SYM_CLASS_BIT_FIELD","A bit-field reference."],["IMAGE_SYM_CLASS_BLOCK","A .bb (beginning of block) or .eb (end of block) record."],["IMAGE_SYM_CLASS_CLR_TOKEN","A CLR token symbol."],["IMAGE_SYM_CLASS_END_OF_FUNCTION","A special symbol that represents the end of function, for debugging purposes."],["IMAGE_SYM_CLASS_END_OF_STRUCT","An end-of-structure entry."],["IMAGE_SYM_CLASS_ENUM_TAG","An enumerated type tagname entry."],["IMAGE_SYM_CLASS_EXTERNAL","A value that Microsoft tools use for external symbols."],["IMAGE_SYM_CLASS_EXTERNAL_DEF","A symbol that is defined externally."],["IMAGE_SYM_CLASS_FILE","The source-file symbol record."],["IMAGE_SYM_CLASS_FUNCTION","A value that Microsoft tools use for symbol records that define the extent of a function."],["IMAGE_SYM_CLASS_LABEL","A code label that is defined within the module."],["IMAGE_SYM_CLASS_MEMBER_OF_ENUM","A member of an enumeration."],["IMAGE_SYM_CLASS_MEMBER_OF_STRUCT","The structure member."],["IMAGE_SYM_CLASS_MEMBER_OF_UNION","A union member."],["IMAGE_SYM_CLASS_NULL","No assigned storage class."],["IMAGE_SYM_CLASS_REGISTER","A register variable."],["IMAGE_SYM_CLASS_REGISTER_PARAM","A register parameter."],["IMAGE_SYM_CLASS_SECTION","A definition of a section (Microsoft tools use STATIC storage class instead)."],["IMAGE_SYM_CLASS_STATIC","A static symbol."],["IMAGE_SYM_CLASS_STRUCT_TAG","The structure tag-name entry."],["IMAGE_SYM_CLASS_TYPE_DEFINITION","A Typedef entry."],["IMAGE_SYM_CLASS_UNDEFINED_LABEL","A reference to a code label that is not defined."],["IMAGE_SYM_CLASS_UNDEFINED_STATIC","A static data declaration."],["IMAGE_SYM_CLASS_UNION_TAG","The Union tag-name entry."],["IMAGE_SYM_CLASS_WEAK_EXTERNAL","A weak external."],["IMAGE_SYM_DEBUG","The symbol provides general type or debugging information but does not correspond to a section."],["IMAGE_SYM_DTYPE_ARRAY","The symbol is an array of base type."],["IMAGE_SYM_DTYPE_FUNCTION","The symbol is a function that returns a base type."],["IMAGE_SYM_DTYPE_NULL","No derived type; the symbol is a simple scalar variable."],["IMAGE_SYM_DTYPE_POINTER","The symbol is a pointer to base type."],["IMAGE_SYM_DTYPE_SHIFT",""],["IMAGE_SYM_TYPE_BYTE","A byte; unsigned 1-byte integer"],["IMAGE_SYM_TYPE_CHAR","A character (signed byte)"],["IMAGE_SYM_TYPE_DOUBLE","An 8-byte floating-point number"],["IMAGE_SYM_TYPE_DWORD","An unsigned 4-byte integer"],["IMAGE_SYM_TYPE_ENUM","An enumerated type"],["IMAGE_SYM_TYPE_FLOAT","A 4-byte floating-point number"],["IMAGE_SYM_TYPE_INT","A natural integer type (normally 4 bytes in Windows)"],["IMAGE_SYM_TYPE_LONG","A 4-byte signed integer"],["IMAGE_SYM_TYPE_MASK",""],["IMAGE_SYM_TYPE_MOE","A member of enumeration (a specific value)"],["IMAGE_SYM_TYPE_NULL","No type information or unknown base type. Microsoft tools use this setting"],["IMAGE_SYM_TYPE_SHORT","A 2-byte signed integer"],["IMAGE_SYM_TYPE_STRUCT","A structure"],["IMAGE_SYM_TYPE_UINT","An unsigned integer of natural size (normally, 4 bytes)"],["IMAGE_SYM_TYPE_UNION","A union"],["IMAGE_SYM_TYPE_VOID","No valid type; used with void pointers and functions"],["IMAGE_SYM_TYPE_WORD","A word; unsigned 2-byte integer"],["IMAGE_SYM_UNDEFINED","The symbol record is not yet assigned a section. A `value` of zero indicates that a reference to an external symbol is defined elsewhere. A `value` of non-zero is a common symbol with a size that is specified by the `value`."],["IMAGE_WEAK_EXTERN_SEARCH_ALIAS","Indicates that the symbol is an alias for the symbol given by the `tag_index` field."],["IMAGE_WEAK_EXTERN_SEARCH_LIBRARY","Indicates that a library search for the symbol should be performed."],["IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY","Indicates that no library search for the symbol should be performed."]],"struct":[["AuxBeginAndEndFunction","Auxiliary symbol record for symbols with storage class `IMAGE_SYM_CLASS_FUNCTION`."],["AuxFunctionDefinition","Auxiliary symbol record for function definitions."],["AuxSectionDefinition","Auxiliary symbol record for section definitions."],["AuxWeakExternal","Auxiliary symbol record for weak external symbols."],["Symbol","A COFF symbol."],["SymbolIterator","An iterator for COFF symbols."],["SymbolTable","A COFF symbol table."]]};