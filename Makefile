CC=gcc
CFLAGS=-Wall -Wextra -std=c99
TARGET=rurima-cli
SOURCE=rurima-cli.c

$(TARGET): $(SOURCE)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCE)

clean:
	rm -f $(TARGET)

install: $(TARGET)
	chmod +x $(TARGET)
	cp $(TARGET) /usr/local/bin/

.PHONY: clean install
