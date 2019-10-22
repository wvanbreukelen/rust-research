ELF=my_due_project
cargo size --bin $ELF -- -A
cargo objcopy --bin $ELF -- -Obinary /tmp/$ELF.bin