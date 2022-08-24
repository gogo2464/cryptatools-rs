set PY_SRC="./src/api/python3"

echo "Generating bdk.py..."
uniffi-bindgen generate ./src/caesar_number.udl --language python --out-dir ./src/api/python3/out