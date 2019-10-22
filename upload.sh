ELF=my_due_project
cargo size --bin $ELF -- -A
cargo objcopy --bin $ELF -- -Obinary /tmp/$ELF.bin
ls /dev/ | grep cu
stty -f /dev/cu.usbmodem14801 speed 1200 cs8 -cstopb -parenb; sleep 1.0
#bossac --port=cu.usbmodem14801 --info -Ufalse -e -w -v -b /tmp/$ELF.bin -v -R
./tools/bossac-arduino --port=cu.usbmodem14801 --info -Ufalse -e -w -v -b /tmp/$ELF.bin -R