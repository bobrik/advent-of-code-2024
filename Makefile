PROBLEMS = ${wildcard problem-*}
BUILD_PROBLEMS = ${subst problem, build-problem, $(PROBLEMS)}
TEST_PROBLEMS = ${subst problem, test-problem, $(PROBLEMS)}
RUN_PROBLEMS = ${subst problem, run-problem, $(PROBLEMS)}

.PHONY: build-problem-%
build-problem-%: problem-%
	@cd $< && cargo build --quiet --release

.PHONY: build
build: $(BUILD_PROBLEMS)

.PHONY: test-problem-%
test-problem-%: problem-%
	@cd $< && cargo test --release && cargo clippy -- -D warnings

.PHONY: test
test: $(TEST_PROBLEMS)

.PHONY: run-problem-%
run-problem-%: problem-% build-problem-%
	@echo $<: $(shell cd $< && ./target/release/$< < input.txt)

.PHONY: run
run: $(RUN_PROBLEMS)

.PHONY: clean
clean:
	rm -rf */target */perf.data */perf.data.old */flamegraph.svg

.PHONY: shell
shell:
	nix develop

.PHONY: timings
timings:
	@$(MAKE) --quiet run | sed --unbuffered -e 's/us]//' -e 's/\[//' | awk '{ if ($$4 > 100000) { color = 31 } else if ($$4 > 10000) { color = 35 } else if ($$4 > 1000) { color = 33 } else { color = 32 }; printf("%s \033[%dm%10.3fms\033[0m ", $$1, color, $$4 / 1000); if (NR % 5 == 0) { printf("\n") } }'
	@echo
