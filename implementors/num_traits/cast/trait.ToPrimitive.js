(function() {var implementors = {
"fraction":[["impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"fraction/trait.Integer.html\" title=\"trait fraction::Integer\">Integer</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"fraction/trait.ToPrimitive.html\" title=\"trait fraction::ToPrimitive\">ToPrimitive</a>&gt; <a class=\"trait\" href=\"fraction/trait.ToPrimitive.html\" title=\"trait fraction::ToPrimitive\">ToPrimitive</a> for <a class=\"enum\" href=\"fraction/prelude/enum.GenericFraction.html\" title=\"enum fraction::prelude::GenericFraction\">GenericFraction</a>&lt;T&gt;"],["impl&lt;T, P&gt; <a class=\"trait\" href=\"fraction/trait.ToPrimitive.html\" title=\"trait fraction::ToPrimitive\">ToPrimitive</a> for <a class=\"struct\" href=\"fraction/prelude/struct.GenericDecimal.html\" title=\"struct fraction::prelude::GenericDecimal\">GenericDecimal</a>&lt;T, P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"fraction/generic/trait.GenericInteger.html\" title=\"trait fraction::generic::GenericInteger\">GenericInteger</a> + <a class=\"trait\" href=\"fraction/trait.ToPrimitive.html\" title=\"trait fraction::ToPrimitive\">ToPrimitive</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"fraction/generic/trait.GenericInteger.html\" title=\"trait fraction::generic::GenericInteger\">GenericInteger</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.usize.html\">usize</a>&gt;,</span>"],["impl&lt;T, G&gt; <a class=\"trait\" href=\"fraction/trait.ToPrimitive.html\" title=\"trait fraction::ToPrimitive\">ToPrimitive</a> for <a class=\"enum\" href=\"fraction/dynaint/enum.DynaInt.html\" title=\"enum fraction::dynaint::DynaInt\">DynaInt</a>&lt;T, G&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"fraction/generic/trait.GenericInteger.html\" title=\"trait fraction::generic::GenericInteger\">GenericInteger</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;G&gt; + <a class=\"trait\" href=\"fraction/convert/trait.TryToConvertFrom.html\" title=\"trait fraction::convert::TryToConvertFrom\">TryToConvertFrom</a>&lt;G&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"fraction/trait.ToPrimitive.html\" title=\"trait fraction::ToPrimitive\">ToPrimitive</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"fraction/generic/trait.GenericInteger.html\" title=\"trait fraction::generic::GenericInteger\">GenericInteger</a>,</span>"]],
"num":[],
"num_bigint":[["impl <a class=\"trait\" href=\"num_traits/cast/trait.ToPrimitive.html\" title=\"trait num_traits::cast::ToPrimitive\">ToPrimitive</a> for <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>"],["impl <a class=\"trait\" href=\"num_traits/cast/trait.ToPrimitive.html\" title=\"trait num_traits::cast::ToPrimitive\">ToPrimitive</a> for <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>"]],
"num_complex":[["impl&lt;T:&nbsp;<a class=\"trait\" href=\"num_traits/cast/trait.ToPrimitive.html\" title=\"trait num_traits::cast::ToPrimitive\">ToPrimitive</a> + <a class=\"trait\" href=\"num_traits/trait.Num.html\" title=\"trait num_traits::Num\">Num</a>&gt; <a class=\"trait\" href=\"num_traits/cast/trait.ToPrimitive.html\" title=\"trait num_traits::cast::ToPrimitive\">ToPrimitive</a> for <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt;"]],
"num_traits":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()