(function() {var implementors = {};
implementors["bincode"] = [{"text":"impl&lt;'de, 'a, R, O&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a mut <a class=\"struct\" href=\"bincode/de/struct.Deserializer.html\" title=\"struct bincode::de::Deserializer\">Deserializer</a>&lt;R, O&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"bincode/de/read/trait.BincodeRead.html\" title=\"trait bincode::de::read::BincodeRead\">BincodeRead</a>&lt;'de&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;O: <a class=\"trait\" href=\"bincode/config/trait.Options.html\" title=\"trait bincode::config::Options\">Options</a>,&nbsp;</span>","synthetic":false,"types":["bincode::de::Deserializer"]}];
implementors["serde"] = [];
implementors["serde_json"] = [{"text":"impl&lt;'de, 'a, R:&nbsp;<a class=\"trait\" href=\"serde_json/de/trait.Read.html\" title=\"trait serde_json::de::Read\">Read</a>&lt;'de&gt;&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a mut <a class=\"struct\" href=\"serde_json/struct.Deserializer.html\" title=\"struct serde_json::Deserializer\">Deserializer</a>&lt;R&gt;","synthetic":false,"types":["serde_json::de::Deserializer"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"enum\" href=\"serde_json/enum.Value.html\" title=\"enum serde_json::Value\">Value</a>","synthetic":false,"types":["serde_json::value::Value"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'de <a class=\"enum\" href=\"serde_json/enum.Value.html\" title=\"enum serde_json::Value\">Value</a>","synthetic":false,"types":["serde_json::value::Value"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"struct\" href=\"serde_json/value/struct.Number.html\" title=\"struct serde_json::value::Number\">Number</a>","synthetic":false,"types":["serde_json::number::Number"]},{"text":"impl&lt;'de, 'a&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'a <a class=\"struct\" href=\"serde_json/value/struct.Number.html\" title=\"struct serde_json::value::Number\">Number</a>","synthetic":false,"types":["serde_json::number::Number"]}];
implementors["toml"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for <a class=\"enum\" href=\"toml/value/enum.Value.html\" title=\"enum toml::value::Value\">Value</a>","synthetic":false,"types":["toml::value::Value"]},{"text":"impl&lt;'de, 'b&gt; <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt; for &amp;'b mut <a class=\"struct\" href=\"toml/de/struct.Deserializer.html\" title=\"struct toml::de::Deserializer\">Deserializer</a>&lt;'de&gt;","synthetic":false,"types":["toml::de::Deserializer"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()