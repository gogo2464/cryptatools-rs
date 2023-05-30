var searchIndex = JSON.parse('{\
"cryptatools_core":{"doc":"","t":"AAFFFFOAAAAAAAAAAADDHLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLADLLLLLLLLLLLLLLAAADLLLLLLLLLLLLLLDLLLLLLLLLLLLLLLLAAAAAAADMLLLLLLLLLLLLLLLLADDMMLLLLLLLLLLLLLLLLLLLLMLLLLLLMLLADLLLLLLLLLLLLLLLAAAADDRRLLLLLLMLLLLLLLLLLMLLLLLLLLLLLFMLLLLLLLLFLLLLDDLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["cryptanalysis","cryptography","ffi_cryptatools_a20e_rustbuffer_alloc","ffi_cryptatools_a20e_rustbuffer_free","ffi_cryptatools_a20e_rustbuffer_from_bytes","ffi_cryptatools_a20e_rustbuffer_reserve","uniffi_reexport_scaffolding","utils","chosen_plain_text_attacks","general_cryptanalysis_methods","know_plaintext_attacks","plain_text_detector","brute_force","frequency_analysis","hash_cryptanalysis","caesar_number","coincidence_index","distribution_algorithms","CoincidenceIndexGenerator","CoincidenceIndexGuesser","ENGLISH_DEFAULT_COINCIDENCE_INDEX","borrow","borrow","borrow_mut","borrow_mut","deref","deref","deref_mut","deref_mut","drop","drop","from","from","generate_coincidence_index_for_key","generate_coincidence_index_for_key_from_file","guess_coincidence_index","guess_coincidence_index_statistics_from_file","init","init","into","into","new","new","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","statistical","Statistical","borrow","borrow_mut","deref","deref_mut","drop","from","guess_statistical_distribution","init","into","new","try_from","try_into","type_id","vzip","birthday_paradox","matsui_s_algorithm","algorithm_one","AlgorithmOne","borrow","borrow_mut","deref","deref_mut","drop","from","init","into","new","solve","try_from","try_into","type_id","vzip","PlainTextDetector","borrow","borrow_mut","catch_confidence_values","deref","deref_mut","detect_language","drop","from","init","into","is_plain_text","new","try_from","try_into","type_id","vzip","classical","modern","encryption","monoalphabetic_ciphers","polyalphabetic_ciphers","transpositional_ciphers","caesar_number","CaesarNumberAlgorithm","alphabet","borrow","borrow_mut","decrypt_by_opcode_shift","deref","deref_mut","drop","encrypt_by_alphabet_shift","encrypt_by_opcode_shift","from","init","into","new","try_from","try_into","type_id","vzip","vigenere","Vigenere","VigenereNoTable","alphabet","alphabet","borrow","borrow","borrow_mut","borrow_mut","deref","deref","deref_mut","deref_mut","drop","drop","encrypt","encrypt","from","from","init","init","into","into","new","new","sorted_alphabet","try_from","try_from","try_into","try_into","type_id","type_id","vigenere_table","vzip","vzip","columnar_transposition","ColumnarTranspositionAlgorithm","borrow","borrow_mut","decrypt","deref","deref_mut","drop","encrypt","from","init","into","new","try_from","try_into","type_id","vzip","encoding","encryption","alphabets","convert","Alphabet","Encoding","PRINTABLE","UU_ENCODING_ALPHABET","ascii_encoding","ascii_printable_only_encoding","borrow","borrow","borrow_mut","borrow_mut","bytes","clone","clone","clone_into","clone_into","deref","deref","deref_mut","deref_mut","drop","drop","encoding","from","from","get_encoding","init","init","intel_x86_32_encoding","into","into","new","new_empty","pokered_charset_encoding","split_bytes_by_characters_representation","str","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","uniffy_opcode_group","unknow_opcodes","uppercase_no_space_ascii_alphabet_encoding","vzip","vzip","Decode","Encode","borrow","borrow","borrow_mut","borrow_mut","deref","deref","deref_mut","deref_mut","drop","drop","encode","from","from","from_ascii_to_u8","from_u8_to_ascii","init","init","into","into","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip"],"q":["cryptatools_core","","","","","","","","cryptatools_core::cryptanalysis","","","","cryptatools_core::cryptanalysis::general_cryptanalysis_methods","","","cryptatools_core::cryptanalysis::general_cryptanalysis_methods::brute_force","cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis","","cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms","cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical","","","","","","","","","","","","","","","cryptatools_core::cryptanalysis::general_cryptanalysis_methods::hash_cryptanalysis","cryptatools_core::cryptanalysis::know_plaintext_attacks","cryptatools_core::cryptanalysis::know_plaintext_attacks::matsui_s_algorithm","cryptatools_core::cryptanalysis::know_plaintext_attacks::matsui_s_algorithm::algorithm_one","","","","","","","","","","","","","","","cryptatools_core::cryptanalysis::plain_text_detector","","","","","","","","","","","","","","","","","cryptatools_core::cryptography","","cryptatools_core::cryptography::classical","cryptatools_core::cryptography::classical::encryption","","","cryptatools_core::cryptography::classical::encryption::monoalphabetic_ciphers","cryptatools_core::cryptography::classical::encryption::monoalphabetic_ciphers::caesar_number","","","","","","","","","","","","","","","","","","cryptatools_core::cryptography::classical::encryption::polyalphabetic_ciphers","cryptatools_core::cryptography::classical::encryption::polyalphabetic_ciphers::vigenere","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cryptatools_core::cryptography::classical::encryption::transpositional_ciphers","cryptatools_core::cryptography::classical::encryption::transpositional_ciphers::columnar_transposition","","","","","","","","","","","","","","","","cryptatools_core::cryptography::modern","","cryptatools_core::utils","","cryptatools_core::utils::alphabets","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cryptatools_core::utils::convert","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","Differential Cryptanalysis","","linear Cryptanalysis","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Generate coincidence index of a specific key_size choosen …","Generate coincidence index corresponding for a sepcific …","Guess coincidence index of <code>cipher_text_input</code>.","Guess coincidence index of an unencrypted plain text …","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Catch statistical distribution (percentage) from a …","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","For each <code>languages</code> set, return a tuple with confidence …","","","Detect the language used in a plain text using the …","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Detect if plain text if the text correspond to a set of …","","","","","","","","","","","","Encrypt with Caesar shifting encryption algorithm.","","Alphabet used by the caesar number encryption Algotithm.","","","Decrypt the cipher text with the caesar number encryption …","","","","Encrypt the plain text with the caesar number encryption …","Encrypt the plain text with the caesar number encryption …","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","Encrypt with the Vigenere algorithm.","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","Decrypt the <code>cipher_text</code> with the columnar transposition …","","","","Encrypt the <code>plain_text</code> with the columnar transposition …","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","Builder to add encoding to the encoding.","","","","","","","","","","","","","","","","Alphabet encoding.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","Unknow opcodes","","","","","","","","","","","","","","","","Encode the input argument <code>unencoded</code> to a byte according …","Returns the argument unchanged.","Returns the argument unchanged.","Encode a string to a vector of u8 bytes.","Decode a vector of u8 to ascii text string.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,11,6,11,6,11,6,11,6,11,6,11,6,6,6,11,11,11,6,11,6,11,6,11,6,11,6,11,6,11,6,0,0,16,16,16,16,16,16,16,16,16,16,16,16,16,16,0,0,0,0,18,18,18,18,18,18,18,18,18,18,18,18,18,18,0,19,19,19,19,19,19,19,19,19,19,19,19,19,19,19,19,0,0,0,0,0,0,0,0,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,0,0,0,25,26,25,26,25,26,25,26,25,26,25,26,25,26,25,26,25,26,25,26,25,26,26,25,26,25,26,25,26,25,25,26,0,0,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0,0,0,0,0,0,0,0,12,12,28,12,28,12,28,28,12,28,12,28,12,28,12,28,12,12,28,12,12,28,12,12,28,12,12,12,12,0,28,28,12,28,12,28,12,28,12,0,12,12,28,12,0,0,29,30,29,30,29,30,29,30,29,30,30,29,30,30,29,29,30,29,30,29,30,29,30,29,30,29,30],"f":[0,0,[[1,2],3],[[3,2]],[[4,2],3],[[3,1,2],3],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[5],[5],[5],[5],[5],[5],[[]],[[]],[[6,5,[8,[7]]],9],[[6,5,10],9],[[11,[8,[7]]],9],[[11,10],9],[[],5],[[],5],[[]],[[]],[[[13,[12]]],11],[[[13,[12]]],6],[[],14],[[],14],[[],14],[[],14],[[],15],[[],15],[[]],[[]],0,0,[[]],[[]],[5],[5],[5],[[]],[[16,[8,[7]]],[[17,[[8,[7]],9]]]],[[],5],[[]],[12,16],[[],14],[[],14],[[],15],[[]],0,0,0,0,[[]],[[]],[5],[5],[5],[[]],[[],5],[[]],[12,18],[[]],[[],14],[[],14],[[],15],[[]],0,[[]],[[]],[[19,10,[8,[20]]],[[21,[8]]]],[5],[5],[[19,10,[8,[20]]],[[21,[20]]]],[5],[[]],[[],5],[[]],[[19,10,[8,[20]],9],22],[[],19],[[],14],[[],14],[[],15],[[]],0,0,0,0,0,0,0,0,0,[[]],[[]],[[23,[8,[7]],24],[[8,[7]]]],[5],[5],[5],[[23,[8,[7]],24],[[8,[7]]]],[[23,[8,[7]],24],[[8,[7]]]],[[]],[[],5],[[]],[[[13,[12]]],23],[[],14],[[],14],[[],15],[[]],0,0,0,0,0,[[]],[[]],[[]],[[]],[5],[5],[5],[5],[5],[5],[[25,[8,[7]],[8,[[8,[7]]]]],[[8,[7]]]],[[26,[8,[7]],[8,[[8,[7]]]]],[[8,[7]]]],[[]],[[]],[[],5],[[],5],[[]],[[]],[12,25],[[[13,[12]]],26],0,[[],14],[[],14],[[],14],[[],14],[[],15],[[],15],0,[[]],[[]],0,0,[[]],[[]],[[27,[8,[7]],24],[[8,[7]]]],[5],[5],[5],[[27,[8,[7]],24],[[8,[7]]]],[[]],[[],5],[[]],[[[13,[12]]],27],[[],14],[[],14],[[],15],[[]],0,0,0,0,0,0,0,0,[12,12],[12,12],[[]],[[]],[[]],[[]],0,[28,28],[12,12],[[]],[[]],[5],[5],[5],[5],[5],[5],0,[[]],[[]],[12,[[8,[28]]]],[[],5],[[],5],[12,12],[[]],[[]],[[[8,[28]]],12],[[],12],[12,12],[[12,[8,[7]]],[[8,[[8,[7]]]]]],0,[[]],[[]],[[],14],[[],14],[[],14],[[],14],[[],15],[[],15],[[[8,[[8,[7]]]]],[[8,[7]]]],[12,12],[12,12],[[]],[[]],0,0,[[]],[[]],[[]],[[]],[5],[5],[5],[5],[5],[5],[[12,10],[[8,[7]]]],[[]],[[]],[10,[[8,[7]]]],[[[8,[7]]],10],[[],5],[[],5],[[]],[[]],[[],14],[[],14],[[],14],[[],14],[[],15],[[],15],[[]],[[]]],"p":[[15,"i32"],[3,"RustCallStatus"],[3,"RustBuffer"],[3,"ForeignBytes"],[15,"usize"],[3,"CoincidenceIndexGenerator"],[15,"u8"],[3,"Vec"],[15,"f64"],[3,"String"],[3,"CoincidenceIndexGuesser"],[3,"Alphabet"],[3,"Arc"],[4,"Result"],[3,"TypeId"],[3,"Statistical"],[3,"HashMap"],[3,"AlgorithmOne"],[3,"PlainTextDetector"],[4,"Language"],[4,"Option"],[15,"bool"],[3,"CaesarNumberAlgorithm"],[15,"u32"],[3,"Vigenere"],[3,"VigenereNoTable"],[3,"ColumnarTranspositionAlgorithm"],[3,"Encoding"],[3,"Decode"],[3,"Encode"]]},\
"uniffi_bindgen":{"doc":"","t":"F","n":["main"],"q":["uniffi_bindgen"],"d":[""],"i":[0],"f":[[[]]],"p":[]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
