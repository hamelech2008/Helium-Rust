PREFIX ?= /usr/local/helium

BIN_DIR = /usr/local/bin
LIB_DIR = $(PREFIX)/lib
INCLUDE_DIR = $(PREFIX)/fetch

BUILD_DIR = build

AS = as
LD = ld

BUILTIN_LIBS := $(patsubst builtin/%/,%,$(wildcard builtin/*/))
LIB_TARGETS := $(addprefix $(BUILD_DIR)/lib,$(addsuffix .so,$(BUILTIN_LIBS)))

.PHONY: all compiler libraries install uninstall clean

all: compiler libraries

# build the rust compiler
compiler:
	cargo build --release

# assemble all the builtin libraries to their .so files
libraries: $(LIB_TARGETS)

# generate build rules for each library
define LIBRARY_template

SOURCES_$(1) := $$(wildcard builtin/$(1)/src/*.s)
OBJECTS_$(1) := $$(patsubst builtin/$(1)/src/%.s,$(BUILD_DIR)/$(1)/%.o,$$(SOURCES_$(1)))

$(BUILD_DIR)/$(1)/%.o: builtin/$(1)/src/%.s
	mkdir -p $$(@D)
	$$(AS) -o $$@ $$<

$(BUILD_DIR)/lib$(1).so: $$(OBJECTS_$(1))
	$$(LD) -shared -o $$@ $$^

endef

$(foreach lib,$(BUILTIN_LIBS),$(eval $(call LIBRARY_template,$(lib))))

# install compiler, libraries and helium declarations
install: all
	install -d $(LIB_DIR) $(INCLUDE_DIR)

	install -m 755 target/release/Helium $(BIN_DIR)/helium

	install -m 755 $(LIB_TARGETS) $(LIB_DIR)/

	@for dir in builtin/*/fetch; do \
		if [ -d "$$dir" ]; then \
			cp -a "$$dir"/. $(INCLUDE_DIR)/; \
		fi \
	done

# remove installed Helium toolchain (libraries, binaries and fetched files)
uninstall:
	rm -rf $(PREFIX)

# remove build artifacts
clean:
	cargo clean
	rm -rf $(BUILD_DIR)