# If you have compile linking errors, uncomment the following:
# sed -i '/	$(CC) $(OBJECTS) $(LDFLAGS) -o $@/c\	$(CC) $(OBJECTS) $(LDFLAGS) ./libcube.a -o $@' MakeFile/CM7/Makefile
# sed -i '/$(BUILD_DIR)\/$(TARGET).elf: $(OBJECTS) Makefile/c\$(BUILD_DIR)\/$(TARGET).elf: $(OBJECTS) Makefile libcube.a ../../lib/lib.rs' MakeFile/CM7/Makefile
# sed -i '/LDFLAGS = $(MCU) -specs=nano.specs -T$(LDSCRIPT) $(LIBDIR) $(LIBS) -Wl,-Map=$(BUILD_DIR)\/$(TARGET).map,--cref -Wl,--gc-sections/c\LDFLAGS = $(MCU) -specs=nano.specs -specs=nosys.specs -T$(LDSCRIPT) $(LIBDIR) $(LIBS) -Wl,-Map=$(BUILD_DIR)/$(TARGET).map,--cref -Wl,--gc-sections' MakeFile/CM7/Makefile

# thumbv7em-none-eabi: Cortex7 No floating point hardware
# thumbv7em-none-eabihf: Cortex7 Floating point hardware
echo Compiling Rust Library Wrapper
cargo build --release --target thumbv7em-none-eabihf
sudo cp target/thumbv7em-none-eabihf/release/libcube.a Makefile/CM7/libcube.a

echo "\n\n\n\n\n\n"
echo Compiling STM C Code
(cd Makefile; make)

echo "\n\n\n\n\n\n"
echo Moving Binary to cube/output/cube_CM7.bin
sudo cp Makefile/CM7/build/cube_CM7.bin output/cube_CM7.bin
