echo "$(cat shellcode.txt | tr -d 'x' | tr -d '\\' | tr -d '\n')" > opcode.txt
xxd -r -p opcode.txt bin
