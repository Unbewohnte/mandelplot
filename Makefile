all:
	cargo build --release && mv ./target/release/mandelplot .

cross:
	rm -rf release/
	mkdir release
	mkdir release/mandelplot_linux_amd64
	mkdir release/mandelplot_windows_amd64
	cp LICENSE release/mandelplot_windows_amd64/
	cp LICENSE release/mandelplot_linux_amd64/

	cargo build --release --target=x86_64-unknown-linux-musl
	mv ./target/x86_64-unknown-linux-musl/release/mandelplot ./release/mandelplot_linux_amd64

	cargo build --release --target=x86_64-pc-windows-gnu
	mv ./target/x86_64-pc-windows-gnu/release/mandelplot.exe ./release/mandelplot_windows_amd64

	cd release/ && zip -r mandelplot_linux_amd64 mandelplot_linux_amd64/
	cd release/ && zip -r mandelplot_windows_amd64 mandelplot_windows_amd64/