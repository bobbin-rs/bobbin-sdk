.PHONY: dsl mcu-src mcu board-src board app periph-src periph

all: dsl mcu board app periph

dsl:
	make -C dsl/

mcu-src:
	make -C mcu-src/

mcu:
	make -C mcu/

board-src:
	make -C board-src/

board:
	make -C board/

app:
	make -C app/

periph-src:
	make -C periph-src/

periph:
	make -C periph/
	