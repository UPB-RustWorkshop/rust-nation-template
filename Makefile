.PHONY: kernel
kernel: 
	cd kernel/pico_explorer_base && make flash
	mkdir -p build
	cp kernel/tock/target/thumbv6m-none-eabi/release/pico_explorer_base.uf2 build/kernel.uf2

.PHONY: app
apps:
	cd applications/c/ui && make
	cd applications/rust/service && make raspberry_pi_pico
	mkdir -p build
	cat applications/c/ui/build/cortex-m0/cortex-m0.tbf applications/rust/service/target/tbf/raspberry_pi_pico/service.tbf > build/kernel-with-app.data
	cd kernel/pico_explorer_base && APP=../../build/kernel-with-app.data make program
	cp kernel/tock/target/thumbv6m-none-eabi/release/pico_explorer_base-app.uf2 build/kernel-with-app.uf2 
	rm build/*.data
