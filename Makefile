.PHONY: all clean fclean run

all: build

build:
	cargo build

clean:
	cargo clean

fclean: clean

re: fclean all

