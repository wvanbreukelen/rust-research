ELF=my_due_project
TARGET=thumbv7m-none-eabi
SERIAL_PORT=cu.usbmodem14E01 #COM10
EXTRA_COMPILE_ARGS="--release" # Might be using --release flag.
#cargo size --bin $ELF -- -A
#cargo objcopy --bin $ELF -- -Obinary /tmp/$ELF.bin

#bash -c "./compile.sh"

cargo size $EXTRA_COMPILE_ARGS --target $TARGET --bin  $ELF -- -A
cargo objcopy $EXTRA_COMPILE_ARGS --target $TARGET --bin $ELF -- -Obinary build/$ELF.bin

#cargo build --target $TARGET --release

ls /dev/ | grep cu
#stty -f /dev/cu.usbmodem14801 speed 1200 cs8 -cstopb -parenb; sleep 1.0

echo "Uploading to $SERIAL_PORT..."

#if [[ "$OSTYPE" == "win32" ]]; then
        #cmd.exe /c @mode $SERIAL_PORT:1200
#elif [[ "$OSTYPE" == "msys"* ]]; then
        # Mac OSX
        # stty -F $SERIAL_PORT speed 1200 cs8 -cstopb -parenb; sleep 1.0
        #cmd.exe /c @mode $SERIAL_PORT:1200
#elif [[ "$OSTYPE" == "cygwin" ]]; then
        # POSIX compatibility layer and Linux environment emulation for Windows
        #stty -F $SERIAL_PORT speed 1200 cs8 -cstopb -parenb; sleep 1.0
#else
    #stty -F $SERIAL_PORT speed 1200 cs8 -cstopb -parenb; sleep 1.0
#fi

stty -f /dev/$SERIAL_PORT speed 1200 cs8 -cstopb -parenb; sleep 1.0
#cmd.exe /c @mode $SERIAL_PORT:1200
#sleep 1.0

#bossac --port=cu.usbmodem14801 --info -Ufalse -e -w -v -b /tmp/$ELF.bin -v -R
if [[ "$OSTYPE" == "win32" ]]; then
    ./tools/bossac-arduino.exe --port=$SERIAL_PORT --info -Ufalse -e -w -v -b build/$ELF.bin -R
elif [[ "$OSTYPE" == "msys"* ]]; then
 ./tools/bossac-arduino.exe --port=$SERIAL_PORT --info -Ufalse -e -w -v -b build/$ELF.bin -R
else
    ./tools/bossac-arduino --port=$SERIAL_PORT --info -Ufalse -e -w -v -b build/$ELF.bin -R
fi

sleep 1.0

#screen /dev/$SERIAL_PORT 9600 –L