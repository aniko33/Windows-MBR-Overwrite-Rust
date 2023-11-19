<div align="center">

  # Rust MBR Overwrite
  Code that overwrites the Windows MBR with a custom bootloader! (Rust edition)

</div>

---

**NOTES: YOU NEED EXECUTE THIS CODE WITH ELEVATED PRIVLAGES**

- src/main.cpp - The Rust file that does the overwriting magic
- boot.asm - The Assembly file that contains the code for the custom bootloader to replace Windows MBR

## How compile ASM code
`nasm boot.asm -o boot.bin`
### Check if file signature
`file boot.bin`

```
mbr.bin: DOS/MBR boot sector
```

How to repair the MBR:
https://neosmart.net/wiki/fix-mbr
