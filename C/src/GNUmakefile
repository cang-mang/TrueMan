MY_CC := rustc --crate-type staticlib -C debuginfo=2
MY_RM := rm -rf
MY_NM := trueman
MY_DL := lib$(MY_NM).a

$(MY_DL): lib.rs
	$(MY_CC) --crate-name $(MY_NM) -o $@ $<

clean:
	$(MY_RM) $(MY_DL)
