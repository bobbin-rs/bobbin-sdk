.PHONY: dsl mcu-src app board-src board

all: dsl mcu-src mcu app board-src board

dsl:
	make -C dsl/

mcu-src:
	make -C mcu-src/

mcu:
	make -C mcu/

app:
	make -C app/

board-src:
	make -C board-src/

board:
	make -C board/