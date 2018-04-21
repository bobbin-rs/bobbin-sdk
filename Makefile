.PHONY: dsl mcu app board

all: dsl mcu app board

dsl:
	make -C dsl/

mcu:
	make -C mcu/

app:
	make -C app/

board:
	make -C board/