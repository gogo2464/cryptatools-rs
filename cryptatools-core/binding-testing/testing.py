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



from cryptatools_core.utils.alphabets import Encoding, Alphabet

a = Alphabet.full_hexadecimal_alphabet()
hex_alphabet = [{d.str: d.bytes} for d in a.get_encoding()]
assert hex_alphabet == [{'00': [0]}, {'01': [1]}, {'02': [2]}, {'03': [3]}, {'04': [4]}, {'05': [5]}, {'06': [6]}, {'07': [7]}, {'08': [8]}, {'09': [9]}, {'0a': [10]}, {'0b': [11]}, {'0c': [12]}, {'0d': [13]}, {'0e': [14]}, {'0f': [15]}, {'10': [16]}, {'11': [17]}, {'12': [18]}, {'13': [19]}, {'14': [20]}, {'15': [21]}, {'16': [22]}, {'17': [23]}, {'18': [24]}, {'19': [25]}, {'1a': [26]}, {'1b': [27]}, {'1c': [28]}, {'1d': [29]}, {'1e': [30]}, {'1f': [31]}, {'20': [32]}, {'21': [33]}, {'22': [34]}, {'23': [35]}, {'24': [36]}, {'25': [37]}, {'26': [38]}, {'27': [39]}, {'28': [40]}, {'29': [41]}, {'2a': [42]}, {'2b': [43]}, {'2c': [44]}, {'2d': [45]}, {'2e': [46]}, {'2f': [47]}, {'30': [48]}, {'31': [49]}, {'32': [50]}, {'33': [51]}, {'34': [52]}, {'35': [53]}, {'36': [54]}, {'37': [55]}, {'38': [56]}, {'39': [57]}, {'3a': [58]}, {'3b': [59]}, {'3c': [60]}, {'3d': [61]}, {'3e': [62]}, {'3f': [63]}, {'40': [64]}, {'41': [65]}, {'42': [66]}, {'43': [67]}, {'44': [68]}, {'45': [69]}, {'46': [70]}, {'47': [71]}, {'48': [72]}, {'49': [73]}, {'4a': [74]}, {'4b': [75]}, {'4c': [76]}, {'4d': [77]}, {'4e': [78]}, {'4f': [79]}, {'50': [80]}, {'51': [81]}, {'52': [82]}, {'53': [83]}, {'54': [84]}, {'55': [85]}, {'56': [86]}, {'57': [87]}, {'58': [88]}, {'59': [89]}, {'5a': [90]}, {'5b': [91]}, {'5c': [92]}, {'5d': [93]}, {'5e': [94]}, {'5f': [95]}, {'60': [96]}, {'61': [97]}, {'62': [98]}, {'63': [99]}, {'64': [100]}, {'65': [101]}, {'66': [102]}, {'67': [103]}, {'68': [104]}, {'69': [105]}, {'6a': [106]}, {'6b': [107]}, {'6c': [108]}, {'6d': [109]}, {'6e': [110]}, {'6f': [111]}, {'70': [112]}, {'71': [113]}, {'72': [114]}, {'73': [115]}, {'74': [116]}, {'75': [117]}, {'76': [118]}, {'77': [119]}, {'78': [120]}, {'79': [121]}, {'7a': [122]}, {'7b': [123]}, {'7c': [124]}, {'7d': [125]}, {'7e': [126]}, {'7f': [127]}, {'80': [128]}, {'81': [129]}, {'82': [130]}, {'83': [131]}, {'84': [132]}, {'85': [133]}, {'86': [134]}, {'87': [135]}, {'88': [136]}, {'89': [137]}, {'8a': [138]}, {'8b': [139]}, {'8c': [140]}, {'8d': [141]}, {'8e': [142]}, {'8f': [143]}, {'90': [144]}, {'91': [145]}, {'92': [146]}, {'93': [147]}, {'94': [148]}, {'95': [149]}, {'96': [150]}, {'97': [151]}, {'98': [152]}, {'99': [153]}, {'9a': [154]}, {'9b': [155]}, {'9c': [156]}, {'9d': [157]}, {'9e': [158]}, {'9f': [159]}, {'a0': [160]}, {'a1': [161]}, {'a2': [162]}, {'a3': [163]}, {'a4': [164]}, {'a5': [165]}, {'a6': [166]}, {'a7': [167]}, {'a8': [168]}, {'a9': [169]}, {'aa': [170]}, {'ab': [171]}, {'ac': [172]}, {'ad': [173]}, {'ae': [174]}, {'af': [175]}, {'b0': [176]}, {'b1': [177]}, {'b2': [178]}, {'b3': [179]}, {'b4': [180]}, {'b5': [181]}, {'b6': [182]}, {'b7': [183]}, {'b8': [184]}, {'b9': [185]}, {'ba': [186]}, {'bb': [187]}, {'bc': [188]}, {'bd': [189]}, {'be': [190]}, {'bf': [191]}, {'c0': [192]}, {'c1': [193]}, {'c2': [194]}, {'c3': [195]}, {'c4': [196]}, {'c5': [197]}, {'c6': [198]}, {'c7': [199]}, {'c8': [200]}, {'c9': [201]}, {'ca': [202]}, {'cb': [203]}, {'cc': [204]}, {'cd': [205]}, {'ce': [206]}, {'cf': [207]}, {'d0': [208]}, {'d1': [209]}, {'d2': [210]}, {'d3': [211]}, {'d4': [212]}, {'d5': [213]}, {'d6': [214]}, {'d7': [215]}, {'d8': [216]}, {'d9': [217]}, {'da': [218]}, {'db': [219]}, {'dc': [220]}, {'dd': [221]}, {'de': [222]}, {'df': [223]}, {'e0': [224]}, {'e1': [225]}, {'e2': [226]}, {'e3': [227]}, {'e4': [228]}, {'e5': [229]}, {'e6': [230]}, {'e7': [231]}, {'e8': [232]}, {'e9': [233]}, {'ea': [234]}, {'eb': [235]}, {'ec': [236]}, {'ed': [237]}, {'ee': [238]}, {'ef': [239]}, {'f0': [240]}, {'f1': [241]}, {'f2': [242]}, {'f3': [243]}, {'f4': [244]}, {'f5': [245]}, {'f6': [246]}, {'f7': [247]}, {'f8': [248]}, {'f9': [249]}, {'fa': [250]}, {'fb': [251]}, {'fc': [252]}, {'fd': [253]}, {'fe': [254]}, {'ff': [255]}], f"cipher={hex_alphabet} incorrect alphabet."
print("encoding = {0}. OK!".format(hex_alphabet))