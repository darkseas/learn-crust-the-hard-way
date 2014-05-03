# ex3

sources = ex3 ex4 ex5

all: $(sources)

$(sources): %: %.rs
	rustc $<
	./$@

clean:
	rm $(sources)