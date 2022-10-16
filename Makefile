t3.xtst: t.xtst 
	txl -q -s 2000 -in 0 -i XML/Txl $< -o $@
#	txl -q -s 2000 -in 0 $< XML/Txl/XML.Txl -o $@
junk += t3.xtst

t1.tst: t.tst
	txl -q -s 2000 -in 2 $< -o $@

junk += t1.tst

t.tst: ~/abc/src/main.rs
	tree-sitter parse $< > $@
junk += t.tst

t.xtst: ~/abc/src/main.rs
	tree-sitter parse $< -x -q > $@
junk += t.xtst

clean::
	rm -f $(junk)
