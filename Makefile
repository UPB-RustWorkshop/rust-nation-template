.PHONY: kernel
kernel: 
	@cd kernel/smarthome && make flash
	@mkdir -p build
	@cp kernel/tock/target/thumbv6m-none-eabi/release/smarthome.uf2 build/kernel.uf2
	@echo "Download build/kernel.uf2 and copy it to Pi's USB driver"

.PHONY: app
app:
	@cd applications/c/ui && make
	@cd applications/rust/service && make raspberry_pi_pico
	@mkdir -p build
	@cat applications/c/ui/build/cortex-m0/cortex-m0.tbf applications/rust/service/target/tbf/raspberry_pi_pico/service.tbf > build/kernel-with-app.data
	@cd kernel/smarthome && APP=../../build/kernel-with-app.data make program
	@cp kernel/tock/target/thumbv6m-none-eabi/release/smarthome-app.uf2 build/kernel-with-app.uf2 
	@rm build/*.data
	@echo "Download build/kernel-with-app.uf2 and copy it to Pi's USB driver"
