/*!
Format XML Templating
=====================

Minimal compile time templating for XML in Rust!

The `format_xml!` macro by example accepts an XML-like syntax and transforms it into a `format_args!` invocation.
We say _XML-like_ because due to limitations of the macro system some concessions had to be made, see the examples below.

Examples
--------

### Basic usage

```rust
# use format_xml::format_xml;
let point = (20, 30);
let name = "World";

# let result =
format_xml! {
	<svg width="200" height="200">
		<line x1="0" y1="0" x2={point.0} y2={point.1} stroke="black" stroke-width="2" />
		<text x={point.1} y={point.0}>"Hello '" {name} "'!"</text>
	</svg>
}.to_string()
# ; assert_eq!(result, r#"<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World'!</text></svg>"#);
```

The resulting string is `<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World!'</text></svg>`.

Note how the expression values to be formatted are inlined in the formatting braces.

### Formatting specifiers

```rust
# use format_xml::format_xml;
let value = 42;

# let result =
format_xml! {
	<span data-value={value}>{value;#x?}</span>
}.to_string()
# ; assert_eq!(result, r#"<span data-value="42">0x2a</span>"#);
```

The resulting string is `<span data-value="42">0x2a</span>`.

Due to limitations of macros by example, a semicolon is used to separate the value from the formatting specifiers. The rules for the specifiers are exactly the same as the standard library of Rust.

### Supported tags

```rust
# use format_xml::format_xml;
# let result =
format_xml! {
	<!doctype html>
	<?xml version="1.0" encoding="UTF-8"?>
	<tag-name></tag-name>
	<ns:self-closing-tag />
	<!-- "comment" -->
	<![CDATA["cdata"]]>
}.to_string()
# ; assert_eq!(result, r#"<!doctype html><?xml version="1.0" encoding="UTF-8"?><tag-name></tag-name><ns:self-closing-tag /><!-- comment --><![CDATA[cdata]]>"#);
```

The resulting string is `<!doctype html><?xml version="1.0" encoding="UTF-8"?><open-tag></open-tag><ns:self-closing-tag />`.

### Control flow

```rust
# use format_xml::format_xml;
let switch = true;
let opt = Some("World");
let result: Result<f32, i32> = Err(13);

# let result =
format_xml! {
	if let Some(name) = (opt) {
		<h1>"Hello " {name}</h1>
	}
	else if (switch) {
		<h1>"Hello User"</h1>
	}
	if (switch) {
		match (result) {
			Ok(f) => { <i>{f}</i> }
			Err(i) => { <b>{i}</b> }
		}
		<ul>
		for i in (1..=5) {
			let times_five = i * 5;
			<li>{i}"*5="{times_five}</li>
		}
		</ul>
	}
	else {
		<p>"No contents"</p>
	}
}.to_string()
# ; assert_eq!(result, r#"<h1>Hello World</h1><b>13</b><ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>"#);
```

The resulting string is `<h1>Hello World</h1><ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>`.

Control flow are currently only supported outside tags. They are not supported in attributes. The expressions for `if` and `for` must be surrounded with parentheses due to macro by example limitations.

### Specialised attribute syntax

```rust
# use format_xml::format_xml;
let has_a = true;
let has_b = false;
let make_red = true;

# let result =
format_xml! {
	<div class=["class-a": has_a, "class-b": has_b]><span style=["color: red;": make_red]></span></div>
}.to_string()
# ; assert_eq!(result, r#"<div class="class-a "><span style="color: red; "></span></div>"#);
```

The resulting string is `<div class="class-a "><span style="color: red; "></span></div>`.

Dedicated syntax for fixed set of space delimited attribute values where each element can be conditionally included. This is specifically designed to work with the style and class attributes of html.

Limitations
-----------

This crate is implemented with standard macros by example (`macro_rules!`). Because of this there are various limitations:

* It is not possible to check whether tags are closed by the appropriate closing tag. This crate will happily accept `<open></close>`. It does enforce more simple lexical rules such as rejecting `</tag/>`.

* Escaping of `&<>"'` is not automatic. You can trivially break the structure by including these characters in either the formatting string or formatted values. Avoid untrusted input!

* The formatting specifiers are separated from its value by a semicolon instead of a colon.

* The compiler may complain about macro expansion recursion limit being reached, simply apply the suggested fix and increase the limit. This crate implements a 'tt muncher' which are known to hit these limits.

* Text nodes must be valid Rust literals. Bare words are not supported.

 */

use std::fmt;

mod util;
pub use self::util::*;

mod xml;

/// Implements `std::fmt::Display` for the Fn closure matching fmt's signature.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FnFmt<F: Fn(&mut fmt::Formatter) -> fmt::Result>(pub F);
impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for FnFmt<F> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(self.0)(f)
	}
}
impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Debug for FnFmt<F> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("FnFmt([closure])")
	}
}
