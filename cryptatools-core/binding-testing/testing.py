from cryptatools_core.cryptography.encryption.monoalphabetic_cipher.caesar_number import CaesarNumberAlgorithm
from cryptatools_core.utils.alphabets import Encoding, Alphabet

from cryptatools_core.cryptography.encryption.transpositional_ciphers.columnar_transposition import ColumnarTranspositionAlgorithm

from cryptatools_core.cryptanalysis.general_cryptanalysis_methods.frequency_analysis.coincidence_index import CoincidenceIndexGuesser

printable_alphabet_list = [
    Encoding(" ", [0x20]),
    Encoding("!", [0x21]),
    Encoding("\"", [0x22]),
    Encoding("#", [0x23]),
    Encoding("$", [0x24]),
    Encoding("%", [0x25]),
    Encoding("&", [0x26]),
    Encoding("'", [0x27]),
    Encoding("(", [0x28]),
    Encoding(")", [0x29]),
    Encoding("*", [0x2a]),
    Encoding("+", [0x2b]),
    Encoding(",", [0x2c]),
    Encoding("-", [0x2d]),
    Encoding(".", [0x2e]),
    Encoding("/", [0x2f]),
    Encoding("0", [0x30]),
    Encoding("1", [0x31]),
    Encoding("2", [0x32]),
    Encoding("3", [0x33]),
    Encoding("4", [0x34]),
    Encoding("5", [0x35]),
    Encoding("6", [0x36]),
    Encoding("7", [0x37]),
    Encoding("8", [0x38]),
    Encoding("9", [0x39]),
    Encoding(":", [0x3a]),
    Encoding(";", [0x3b]),
    Encoding("<", [0x3c]),
    Encoding("=", [0x3d]),
    Encoding(">", [0x3e]),
    Encoding("?", [0x3f]),
    Encoding("@", [0x40]),
    Encoding("A", [0x41]),
    Encoding("B", [0x42]),
    Encoding("C", [0x43]),
    Encoding("D", [0x44]),
    Encoding("E", [0x45]),
    Encoding("F", [0x46]),
    Encoding("G", [0x47]),
    Encoding("H", [0x48]),
    Encoding("I", [0x49]),
    Encoding("J", [0x4a]),
    Encoding("K", [0x4b]),
    Encoding("L", [0x4c]),
    Encoding("M", [0x4d]),
    Encoding("N", [0x4e]),
    Encoding("O", [0x4f]),
    Encoding("P", [0x50]),
    Encoding("Q", [0x51]),
    Encoding("R", [0x52]),
    Encoding("S", [0x53]),
    Encoding("T", [0x54]),
    Encoding("U", [0x55]),
    Encoding("V", [0x56]),
    Encoding("W", [0x57]),
    Encoding("X", [0x58]),
    Encoding("Y", [0x59]),
    Encoding("Z", [0x5a]),
    Encoding("[", [0x5b]),
    Encoding("\\", [0x5c]),
    Encoding("],", [0x5d]),
    Encoding("^", [0x5e]),
    Encoding("_", [0x5f]),
    Encoding("`", [0x60]),
    Encoding("a", [0x61]),
    Encoding("b", [0x62]),
    Encoding("c", [0x63]),
    Encoding("d", [0x64]),
    Encoding("e", [0x65]),
    Encoding("f", [0x66]),
    Encoding("g", [0x67]),
    Encoding("h", [0x68]),
    Encoding("i", [0x69]),
    Encoding("j", [0x6a]),
    Encoding("k", [0x6b]),
    Encoding("l", [0x6c]),
    Encoding("m", [0x6d]),
    Encoding("n", [0x6e]),
    Encoding("o", [0x6f]),
    Encoding("p", [0x70]),
    Encoding("q", [0x71]),
    Encoding("r", [0x72]),
    Encoding("s", [0x73]),
    Encoding("t", [0x74]),
    Encoding("u", [0x75]),
    Encoding("v", [0x76]),
    Encoding("w", [0x77]),
    Encoding("x", [0x78]),
    Encoding("y", [0x79]),
    Encoding("z", [0x7a]),
    Encoding("{", [0x7b]),
    Encoding("|", [0x7c]),
    Encoding("}", [0x7d]),
    Encoding("~", [0x7e])
]

printable_alphabet = Alphabet(printable_alphabet_list)
c = CaesarNumberAlgorithm(alphabet=printable_alphabet)

cipher = c.encrypt_by_alphabet_shift([0x61, 0x61], 6)
assert cipher == [0x67, 0x67], f"cipher={cipher} (should be [8, 8])"
print("cipher = {0}. OK!".format(cipher))

cipher = c.encrypt_by_alphabet_shift([0x41, 0x41], 1)
assert cipher == [0x42, 0x42], f"cipher={cipher} (should be [0x42, 0x42])"
print("cipher = {0}. OK!".format(cipher))

cipher = c.encrypt_by_alphabet_shift([0x7e, 0x7e], 3)
assert cipher == [0x22, 0x22], f"cipher={cipher} (should be [0x23, 0x23])"
print("cipher = {0}. OK!".format(cipher))

transpositional_cipher = ColumnarTranspositionAlgorithm(alphabet=printable_alphabet)
plain = transpositional_cipher.encrypt([0x41, 0x42, 0x43, 0x44, 0x45, 0x46], 3)
assert plain == [0x41, 0x44, 0x42, 0x45, 0x43, 0x46], f"cipher={plain} incorrect transposition cipher encryption."
print("plain = {0}. OK!".format(plain))

cig = CoincidenceIndexGuesser(alphabet=printable_alphabet)
plain = cig.guess_coincidence_index([0x41, 0x41, 0x42, 0x41, 0x41, 0x42])
assert plain == 0.4666666666666667, f"cipher={plain} incorrect coincidence index."
print("plain = {0}. OK!".format(plain))











