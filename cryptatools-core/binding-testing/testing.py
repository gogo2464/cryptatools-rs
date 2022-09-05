from cryptatools_core.python3_bindings import CaesarNumberAlgorithm 

c = CaesarNumberAlgorithm(alphabet={'a': [0x61], 'b': [0x62], 'c': [0x63]});

cipher = c.encrypt([0x61, 0x61], 6)

assert cipher == [1, 1], f"cipher={cipher} (should be [1,1])"
print("cipher = {0}. OK!".format(cipher))