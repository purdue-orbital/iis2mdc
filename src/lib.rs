use embedded_hal::i2c::I2c;

const ADDR: u8 = 0b0011110;

pub fn get_ids(bus: &mut impl I2c) -> [u8; 2] {
	let mut buf = [0_u8; 1];

	let res = bus.write_read(ADDR, &[0x4F], &mut buf);

	if res.is_err() {
		log::error!("{:?}", res);
	}

	[buf[0], 0]
}
