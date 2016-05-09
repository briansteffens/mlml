default: build

build:
	cargo build --release

install:
	mkdir -p ${DESTDIR}/usr/bin
	cp target/release/mlml ${DESTDIR}/usr/bin/mlml

clean:
	rm Cargo.lock
	rm -r target
