#!/usr/bin/ruby

require "openssl"

# Define your telegrams here. The more you define, the more confidence we can have it in the discovered key
telegrams =	[
	"3E44A5112655687276077A6ED330055C1B9AB21431F33C04BBE4741DCE827E6849F8407FFCDFB7EFB0262CC350CE8AD13A7B2DE5BE281C5896B6D4E06FDC3A",
	"3E44A5112655687276077A8DD33005C20A86D323D5AD1A25E3BA4E64427E7A1612693F8CFDAC4B79745E68F41BDC99B936F1A8122449061BB9B1D4BECD9406",
	"3E44A5112655687276077A39003005EBAEEB906AE817B45D3CC6B46A955BAD34DEA47B00F860ACBB6D280069A227B334CEE23006878125BBAD10EDADAD9635",
]

decipher = OpenSSL::Cipher.new("aes-128-cbc")
decipher.decrypt
decipher.padding = 0

manufacturer = telegrams[0][4..7]
meter_id = telegrams[0][14..15]+telegrams[0][12..13]+telegrams[0][10..11]+telegrams[0][8..9]
address = telegrams[0][8..19]
access_number1= telegrams[0][22..23]
access_number2 = telegrams[1][22..23]

# Setting the "access_number" in the IV to "00" makes the IV invalid.
# However, the first 14 bytes of the IV are correct and therefore our
# oracle check works (which only checks the first 4 bytes)
decipher.iv = [manufacturer+address+"00"*8].pack("H*")

keys_checked = 0;
start_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)

for i01 in 0..15 do
for i02 in 0..15 do
elapsed = Process.clock_gettime(Process::CLOCK_MONOTONIC) - start_time
print "\r#{100*(i01*16+i02)/(16*16)}%  --  #{(keys_checked/elapsed).round} keys/sec"
for i03 in 0..15 do
for i04 in 0..15 do
for i05 in 0..15 do
for i06 in 0..15 do
for i07 in 0..15 do

	keys_checked += 1
	key = ( i01.to_s(16) + i02.to_s(16) + i03.to_s(16) + i04.to_s(16) + i05.to_s(16) + i06.to_s(16) + i07.to_s(16) + "200012345" ) * 2

	# This block decodes the given telegrams until we have all decoded or until there the magic start sequence
	# does not match (which means the key is wrong).
	i = 0
	begin
		decipher.reset
		decipher.key = [key].pack("H*")

		# Shorten the cipher to one block in order to reduce the amount of data that is decrypted
		plain_text = decipher.update([telegrams[i][30..61]].pack("H*")) + decipher.final

		i += 1
	end while (plain_text.unpack("H*")[0][0,4] == "2f2f") && i < telegrams.length

	# Print the key if all telegrams have the magic start sequence in their plain text
	if (i == telegrams.length)
		puts "---------------------------------------------------------------------------------------------------"
		puts "\n#{key}"

		puts "Run the following to decode the telegrams:"
		telegrams.each do |t|
			puts "docker run -t -i --rm --name=wmbusmeters weetmuts/wmbusmeters /wmbusmeters/wmbusmeters --debug #{t} hydrus auto #{meter_id} #{key}"
		end
	end
end
end
end
end
end
end
end
