(function() {var implementors = {};
implementors["clap"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;<a class=\"enum\" href=\"clap/enum.AppSettings.html\" title=\"enum clap::AppSettings\">AppSettings</a>&gt; for <a class=\"enum\" href=\"clap/enum.AppSettings.html\" title=\"enum clap::AppSettings\">AppSettings</a>","synthetic":false,"types":["clap::build::app_settings::AppSettings"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;<a class=\"enum\" href=\"clap/enum.ArgSettings.html\" title=\"enum clap::ArgSettings\">ArgSettings</a>&gt; for <a class=\"enum\" href=\"clap/enum.ArgSettings.html\" title=\"enum clap::ArgSettings\">ArgSettings</a>","synthetic":false,"types":["clap::build::arg_settings::ArgSettings"]}];
implementors["fraction"] = [{"text":"impl&lt;T, G&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;<a class=\"enum\" href=\"fraction/dynaint/enum.DynaInt.html\" title=\"enum fraction::dynaint::DynaInt\">DynaInt</a>&lt;T, G&gt;&gt; for <a class=\"enum\" href=\"fraction/dynaint/enum.DynaInt.html\" title=\"enum fraction::dynaint::DynaInt\">DynaInt</a>&lt;T, G&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"fraction/generic/trait.GenericInteger.html\" title=\"trait fraction::generic::GenericInteger\">GenericInteger</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;G&gt; + <a class=\"trait\" href=\"fraction/convert/trait.TryToConvertFrom.html\" title=\"trait fraction::convert::TryToConvertFrom\">TryToConvertFrom</a>&lt;G&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.63.0/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;Output = T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"fraction/generic/trait.GenericInteger.html\" title=\"trait fraction::generic::GenericInteger\">GenericInteger</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;Output = G&gt;,&nbsp;</span>","synthetic":false,"types":["fraction::dynaint::DynaInt"]},{"text":"impl&lt;'a, T, G&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;&amp;'a <a class=\"enum\" href=\"fraction/dynaint/enum.DynaInt.html\" title=\"enum fraction::dynaint::DynaInt\">DynaInt</a>&lt;T, G&gt;&gt; for &amp;'a <a class=\"enum\" href=\"fraction/dynaint/enum.DynaInt.html\" title=\"enum fraction::dynaint::DynaInt\">DynaInt</a>&lt;T, G&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"fraction/generic/trait.GenericInteger.html\" title=\"trait fraction::generic::GenericInteger\">GenericInteger</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;G&gt; + <a class=\"trait\" href=\"fraction/convert/trait.TryToConvertFrom.html\" title=\"trait fraction::convert::TryToConvertFrom\">TryToConvertFrom</a>&lt;G&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.63.0/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;Output = T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"fraction/generic/trait.GenericInteger.html\" title=\"trait fraction::generic::GenericInteger\">GenericInteger</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;Output = G&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.63.0/std/primitive.reference.html\">&amp;'a </a>G, Output = G&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.63.0/std/primitive.reference.html\">&amp;'a </a>G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;G, Output = G&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;Output = G&gt;,&nbsp;</span>","synthetic":false,"types":["fraction::dynaint::DynaInt"]}];
implementors["hashbrown"] = [{"text":"impl&lt;T, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;&amp;<a class=\"struct\" href=\"hashbrown/hash_set/struct.HashSet.html\" title=\"struct hashbrown::hash_set::HashSet\">HashSet</a>&lt;T, S, A&gt;&gt; for &amp;<a class=\"struct\" href=\"hashbrown/hash_set/struct.HashSet.html\" title=\"struct hashbrown::hash_set::HashSet\">HashSet</a>&lt;T, S, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Allocator + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["hashbrown::set::HashSet"]}];
implementors["indexmap"] = [{"text":"impl&lt;T, S1, S2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;&amp;<a class=\"struct\" href=\"indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;T, S2&gt;&gt; for &amp;<a class=\"struct\" href=\"indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;T, S1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,&nbsp;</span>","synthetic":false,"types":["indexmap::set::IndexSet"]}];
implementors["num_bigint"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;<a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>&gt; for <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>","synthetic":false,"types":["num_bigint::bigint::BigInt"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;<a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>&gt; for &amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>","synthetic":false,"types":["num_bigint::bigint::BigInt"]},{"text":"impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;&amp;'b <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>&gt; for &amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>","synthetic":false,"types":["num_bigint::bigint::BigInt"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;&amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>&gt; for <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>","synthetic":false,"types":["num_bigint::bigint::BigInt"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;<a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>&gt; for <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>","synthetic":false,"types":["num_bigint::biguint::BigUint"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;<a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>&gt; for &amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>","synthetic":false,"types":["num_bigint::biguint::BigUint"]},{"text":"impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;&amp;'b <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>&gt; for &amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>","synthetic":false,"types":["num_bigint::biguint::BigUint"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.63.0/core/ops/bit/trait.BitOr.html\" title=\"trait core::ops::bit::BitOr\">BitOr</a>&lt;&amp;'a <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>&gt; for <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>","synthetic":false,"types":["num_bigint::biguint::BigUint"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()