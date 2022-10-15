t1.tst: t.tst
	txl -q -s 2000 -in 2 $< -o $@
junk += t1.tst

t.tst: ~/abc/src/main.rs
	tree-sitter parse ~/abc/src/main.rs > $@
junk += t.tst

clean::
	rm -f $(junk)
