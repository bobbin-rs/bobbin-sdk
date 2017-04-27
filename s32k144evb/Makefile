.PHONY: blinky uart clean

blinky:
	xargo build --example blinky
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/debug/examples/blinky blinky.hex
	arm-none-eabi-objdump -s target/thumbv7em-none-eabihf/debug/examples/blinky > sections.txt
	arm-none-eabi-objdump -h target/thumbv7em-none-eabihf/debug/examples/blinky > headers.txt
	arm-none-eabi-objdump -d target/thumbv7em-none-eabihf/debug/examples/blinky > asm.txt

uart:
	xargo build --example uart
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/debug/examples/uart uart.hex
	arm-none-eabi-objdump -s target/thumbv7em-none-eabihf/debug/examples/uart > sections.txt
	arm-none-eabi-objdump -h target/thumbv7em-none-eabihf/debug/examples/uart > headers.txt
	arm-none-eabi-objdump -d target/thumbv7em-none-eabihf/debug/examples/uart > asm.txt	

clean:
	xargo clean