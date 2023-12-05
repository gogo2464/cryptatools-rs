Let's create a python script in order to generate a public key with a wrong exponent:


```shell
pip install rsa
```


```python
import rsa

(pubkey, privkey) = rsa.newkeys(4096, accurate=True, exponent=1)

with open('private.pem', mode='rb') as privatefile:
    keydata = privatefile.read()
privkey = rsa.PrivateKey.load_pkcs1(keydata)
```

