# ArraysRRBVector

An [`Arrays`](https://github.com/Qqwy/elixir-arrays) implementation based on a set of NIFs (Natively Implemented Functions) written in Rust.
The internal representation of the array is an immutable persistent datastructure known as a 'Relaxed Radix Balanced Vector', provided by the [`im`](http://immutable.rs/) library ([`im::Vector`](https://docs.rs/im/15.0.0/im/vector/struct.Vector.html)).

Since the performance of this library seems subpar, it is unlikely that the library will be developed further (unless other approaches to improve performance are discovered).

## Performance

With benchmarking this implementation against the pure-Elixir implementations of `Arrays` (Erlang's `:array` and a Map-based array) we find out that unfortunately the overhead of calling a NIF overshadows any performance gains that would be obtained from using this close-to-the-metal implementation of a RRBVector.

The benchmarks can be run by running `mix run benchmarks/benchmarks.exs`.
Below graphs were constructed from the CSV results.

![random_update](https://github.com/Qqwy/elixir-arrays_rrb_vector/blob/main/benchmark_runs/random_element_update.png)
![random_update](https://github.com/Qqwy/elixir-arrays_rrb_vector/blob/main/benchmark_runs/random_element_update_focus.png)
![random_read](https://github.com/Qqwy/elixir-arrays_rrb_vector/blob/main/benchmark_runs/random_element_read.png)
![random_read](https://github.com/Qqwy/elixir-arrays_rrb_vector/blob/main/benchmark_runs/random_element_read_focus.png)
![append](https://github.com/Qqwy/elixir-arrays_rrb_vector/blob/main/benchmark_runs/append_graph.png)
![append](https://github.com/Qqwy/elixir-arrays_rrb_vector/blob/main/benchmark_runs/append_graph_focus.png)
![append](https://github.com/Qqwy/elixir-arrays_rrb_vector/blob/main/benchmark_runs/append_graph_focus2.png)


## Installation

ArraysRRBVector is [available in Hex](https://hex.pm/docs/publish), and can be installed
by adding `arrays_rrb_vector` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:arrays_rrb_vector, "~> 0.1.0"}
  ]
end
```

Documentation can be found at [https://hexdocs.pm/arrays_rrb_vector](https://hexdocs.pm/arrays_rrb_vector).

