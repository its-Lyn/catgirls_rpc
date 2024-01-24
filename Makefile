.PHONY: install bin_install uninstall build clean

install:
	@mkdir -pv /etc/systemd/user/
	
	cp $(CURDIR)/target/release/catgirl_rpc /usr/local/bin
	cp $(CURDIR)/catgirl_rpc.service /etc/systemd/user/

	@echo "Done! You can now run \"systemctl --user enable --now catgirl_rpc.service\" to enable the rpc when your system starts."

bin_install:
	@mkdir -pv /etc/systemd/user/
	
	cp $(CURDIR)/catgirl_rpc /usr/local/bin
	cp $(CURDIR)/catgirl_rpc.service /etc/systemd/user/

	@echo "Done! You can now run \"systemctl --user enable --now catgirl_rpc.service\" to enable the rpc when your system starts."

uninstall:
	rm /usr/local/bin/catgirl_rpc
	rm /etc/systemd/user/catgirl_rpc.service 

build:
	@cargo build --release

clean:
	@cargo clean
