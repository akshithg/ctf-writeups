.PHONY: all clean

all: clean readme.md

clean:
	rm -f readme.md

readme.md:
	./make-readme.py
