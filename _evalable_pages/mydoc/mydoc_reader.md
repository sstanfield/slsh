---
title: Reader
tags: [documentation]
sidebar: mydoc_sidebar
permalink: mydoc_reader.html
---
# Reader

The reader should work more or less as expected for a lisp reader.

## Strings
String are surrounded by "" and can be multiline.  Stings support the following escape sequences:
- \n: newline
- \r: carriage return
- \t: tab
- \\: backslash
- \xnn: A two digit hex up to 7F, becomes the 7 bit ascii code provided (which is utf8 compatible)
- \u{nnnn} or \unnnn: Second form must be followed by whitespace, embedd a unicode scalar with the value (in hex) of nnnn up to 4 bytes

## String Literals
String literals are started with #"% and end with %".  The % can be any char and must match on both ends.  Anything between the opening #"% and %" will be part of the resulting string (nothing is escaped or expanded).

## Chars
Chars are read using #\x format (for instance 'S' would be #\S). Char supports the following special cases:
- #\space: space character
- #\tab: tab character
- #\newline: newline
- #\linefeed: linefeed
- #\return: carriage return
- #\backspace: backspace
- #\xnn: the 7 bit ascii code (up to 7F) as two hex digits
- #\u{nnnn} or \unnnn: the unicode scalar with the value (in hex) of nnnn up to 4 bytes

## Reader macros
Are supported- add details.

## Vectors
Use the #() syntax to create a vector instead of a list (in sl-sh vectors can be evaluated like lists).

## Comments
Comment to end of line with the ; character.

Block comment with #\|...\|# these can be multiline and can be embedded.

Comment out (read and throwaway) s-expression with #;.  If the sexp after #; is not readable then it will raise an error.  Will call reader macros in order to read the sexp.

## Builtin macros under the \# key
- #\| start block comment
- #\\ character
- #< unreadable, this will error out- used to print things that can not be read back in
- #( read a vector instead of a list (terminates with ')')
- #t true
- #f false
- #. evaluates the next form and replaces itself with that evaluation
- #; read and discard the next sexp (will be an error if the sexp can not be read)
- #"_ read a string literal (see above)
