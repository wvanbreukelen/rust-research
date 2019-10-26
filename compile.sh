ELF=my_due_project
TARGET=thumbv7m-none-eabi
cargo size --release --target $TARGET --bin  $ELF -- -A
cargo objcopy --release --target $TARGET --bin $ELF -- -Obinary /tmp/$ELF.bin