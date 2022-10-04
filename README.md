# array_macro
Testing a rust macro I made\
For everything inside the macro, whenever there are square brackets with semicolons, change array type and array literal syntax from\
[...[[expr; sizeN]; sizeN-1]; ...; size1] to [size1; ...; sizeN-1; sizeN; expr]
