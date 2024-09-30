.PHONY: test
test:
	cargo test

.PHONY:readme
readme:
	@awk -i inplace 'BEGIN { in_section = 0 } \
	/^<!-- help start -->/ { \
	  in_section = 1; \
	  print; \
	  print ""; \
	  print "```console"; \
	  print "$$ thqm --help"; \
	  system("cargo run -- -h"); \
	  print "```"; \
	  print ""; \
	} \
	/^<!-- help end -->/ { in_section = 0 } \
	!in_section' README.md

